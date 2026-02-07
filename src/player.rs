use std::f32::consts::PI;

use bevy::{
    asset::Assets,
    camera::{
        Camera, Camera3d, ClearColorConfig, PerspectiveProjection, Projection,
        visibility::{RenderLayers, Visibility},
    },
    color::{Color, palettes::tailwind},
    core_pipeline::tonemapping::Tonemapping,
    ecs::{
        children,
        component::Component,
        query::{With, Without},
        system::{Commands, Res, ResMut, Single},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::AccumulatedMouseMotion},
    light::NotShadowCaster,
    math::{Dir3, EulerRot, Quat, Vec2, Vec3, Vec3Swizzles, primitives::Cuboid},
    mesh::{Mesh, Mesh3d},
    pbr::{DistanceFog, FogFalloff, MeshMaterial3d, StandardMaterial},
    post_process::bloom::Bloom,
    reflect::Reflect,
    time::Time,
    transform::components::Transform,
    utils::default,
};
use bevy_atmosphere::prelude::Gradient;
use bevy_atmosphere::{model::AtmosphereModel, plugin::AtmosphereCamera};

use avian3d::prelude::{LayerMask, LinearVelocity, RayCaster, RayHits, SpatialQueryFilter};

use crate::settings::{self, Settings};

#[derive(Debug, Component)]
pub struct Player {
    pub movespeed: f32,
}

#[derive(Debug, Component)]
pub struct PlayerCamera;

#[derive(Reflect)]
pub struct PlayerFloorCast;

#[derive(Debug, Component)]
struct WorldModelCamera;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
pub const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
pub const VIEW_MODEL_RENDER_LAYER: usize = 1;

pub const PLAYER_FLOOR_LAYER: u32 = 2;

/// Acceleration in m/s^2
pub const PLAYER_ACCELERATION: f32 = 40.0 / 3.6;

/// Velocity in m/s calculated from km/h
pub const PLAYER_MAX_SPEED: f32 = 4.0 / 3.6;
/// Velocity squared to optimize comparaisons
pub const PLAYER_MAX_SPEED_2: f32 = PLAYER_MAX_SPEED * PLAYER_MAX_SPEED;

// Effective gravity in m/s^2 in Z
pub const PLAYER_BUOYANCY: f32 = -5.;

// Jump velocity
pub const PLAYER_JUMP_IMPULSE: f32 = 0.5;

pub const FLOOR_RAY_PRE_LEN: f32 = 1.0;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    commands.insert_resource(AtmosphereModel::new(Gradient {
        sky: Color::srgb_u8(8, 10, 20).into(),
        horizon: Color::srgb_u8(5, 6, 13).into(),
        ground: Color::srgb_u8(5, 6, 13).into(),
    }));
    let camera = commands
        .spawn((
            PlayerCamera,
            Transform::from_xyz(0.0, 0.0, 1.6), //.looking_to(Vec3::X, Vec3::Z),
            Visibility::default(),
            children![
                (
                    WorldModelCamera,
                    Camera3d::default(),
                    AtmosphereCamera::default(),
                    Camera {
                        clear_color: ClearColorConfig::Custom(Color::srgb_u8(0, 0, 0)),
                        ..default()
                    },
                    Projection::from(PerspectiveProjection {
                        fov: 80.0_f32.to_radians(),
                        ..default()
                    }),
                    Bloom::OLD_SCHOOL,
                    Tonemapping::TonyMcMapface,
                    DistanceFog {
                        color: Color::srgb_u8(5, 6, 13),
                        falloff: FogFalloff::Exponential { density: 0.6 },
                        ..default()
                    },
                ),
                // Spawn view model camera.
                (
                    Camera3d::default(),
                    AtmosphereCamera::default(),
                    Camera {
                        clear_color: ClearColorConfig::Custom(Color::srgb_u8(0, 0, 0)),
                        // Bump the order to render on top of the world model.
                        order: 1,
                        ..default()
                    },
                    Bloom::OLD_SCHOOL,
                    Tonemapping::TonyMcMapface,
                    Projection::from(PerspectiveProjection {
                        fov: 70.0_f32.to_radians(),
                        ..default()
                    }),
                    // Only render objects belonging to the view model.
                    RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                ),
                // Spawn the player's right arm.
                (
                    Mesh3d(arm),
                    MeshMaterial3d(arm_material),
                    Transform::from_xyz(0.2, -0.1, -0.25),
                    // Ensure the arm is only rendered by the view model camera.
                    RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                    // The arm is free-floating, so shadows would look weird.
                    NotShadowCaster,
                ),
            ],
        ))
        .id();

    let playercast = RayCaster::new(Vec3::Z * FLOOR_RAY_PRE_LEN, Dir3::NEG_Z)
        .with_max_distance(FLOOR_RAY_PRE_LEN)
        .with_max_hits(1)
        .with_query_filter(SpatialQueryFilter {
            mask: LayerMask::NONE | PLAYER_FLOOR_LAYER,
            excluded_entities: Default::default(),
        });

    commands
        .spawn((
            Player { movespeed: 2.5 },
            playercast,
            avian3d::dynamics::prelude::RigidBody::Kinematic,
            LinearVelocity::default(),
            Transform::default(),
            Visibility::default(),
        ))
        .add_child(camera);
}

pub fn move_player_camera(
    delta: &Vec2,
    player_transform: &mut Transform,
    camera_transform: &mut Transform,
    sensitivity: f32,
) {
    if delta.x != 0.0 {
        let delta_yaw = -delta.x * sensitivity * 0.01;

        player_transform.rotate_local_z(delta_yaw);
    }
    if delta.y != 0.0 {
        let delta_pitch = -delta.y * sensitivity * 0.01;

        const PITCH_LIMIT: f32 = PI - 0.01;
        camera_transform.rotation = Quat::from_rotation_x(
            (camera_transform.rotation.to_euler(EulerRot::XYZ).0 + delta_pitch)
                .clamp(0.01, PITCH_LIMIT),
        );
    }
}

pub fn get_wishdir(inputs: &ButtonInput<KeyCode>, keys: &settings::Inputs) -> Vec2 {
    let mut wishdir = Vec2::default();

    wishdir.y += inputs
        .pressed(keys.forward)
        .then_some(1.0)
        .unwrap_or_default();
    wishdir.y -= inputs.pressed(keys.back).then_some(1.0).unwrap_or_default();
    wishdir.x += inputs
        .pressed(keys.right)
        .then_some(1.0)
        .unwrap_or_default();
    wishdir.x -= inputs.pressed(keys.left).then_some(1.0).unwrap_or_default();
    wishdir
}

pub fn move_player(
    time: Res<Time>,
    inputs: Res<ButtonInput<KeyCode>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<
        (&mut Transform, &Player, &RayHits, &mut LinearVelocity),
        (With<Player>, Without<PlayerCamera>),
    >,
    camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    settings: Res<Settings>,
) {
    let (mut player_transform, player, floor_hits, mut velocity) = player.into_inner();
    let mut camera_transform = camera.into_inner();

    player_transform.translation += velocity.0 * time.delta_secs();

    move_player_camera(
        &accumulated_mouse_motion.delta,
        &mut player_transform,
        &mut camera_transform,
        settings.camera_sensitivity,
    );

    let mut wishdir = get_wishdir(&inputs, &settings.inputs);

    if wishdir != Vec2::ZERO {
        wishdir = wishdir.normalize();
        wishdir = (player_transform.rotation * Vec3::from((wishdir, 0.0))).xy();
    }

    if let Some(ground) = floor_hits.first() {
        if velocity.0.xy().dot(velocity.0.xy())
            > PLAYER_MAX_SPEED_2 * player.movespeed * player.movespeed
            || wishdir == Vec2::ZERO
        {
            wishdir = -velocity.0.xy();
        }

        if velocity.0.z <= 0.0 {
            velocity.0.z = 0.0;
            if inputs.pressed(settings.inputs.jump) {
                velocity.0.z = PLAYER_JUMP_IMPULSE;
            }
        }
        player_transform.translation.z += (FLOOR_RAY_PRE_LEN - ground.distance) * time.delta_secs();
    } else {
        velocity.0.z += PLAYER_BUOYANCY * time.delta_secs() * time.delta_secs();
    }

    if wishdir != Vec2::ZERO {
        let mut moveforce = wishdir * PLAYER_MAX_SPEED - velocity.0.xy();

        moveforce /= PLAYER_MAX_SPEED;
        moveforce *= PLAYER_ACCELERATION * time.delta_secs() * time.delta_secs();

        if floor_hits.is_empty() {
            moveforce *= 0.1;
        }

        let moveforce = Vec3::from((moveforce, 0.0));

        velocity.0 += moveforce;
        player_transform.translation += (moveforce * time.delta_secs()) / 2.0;
    }
}
