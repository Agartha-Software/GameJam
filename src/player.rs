use std::f32::consts::PI;

use bevy::{
    asset::Assets,
    camera::{
        Camera, Camera3d, PerspectiveProjection, Projection,
        visibility::{RenderLayers, Visibility},
    },
    color::{Color, palettes::tailwind},
    ecs::{
        children,
        component::Component,
        query::{With, Without},
        system::{Commands, Res, ResMut, Single},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::AccumulatedMouseMotion},
    light::NotShadowCaster,
    math::{EulerRot, Quat, Vec2, Vec3, primitives::Cuboid},
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::settings::Settings;

#[derive(Debug, Component)]
pub struct Player {
    pub movespeed: f32,
}

#[derive(Debug, Component)]
pub struct PlayerCamera;

#[derive(Debug, Component)]
struct WorldModelCamera;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
pub const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
pub const VIEW_MODEL_RENDER_LAYER: usize = 1;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    let camera = commands
        .spawn((
            PlayerCamera,
            Transform::from_xyz(0.0, 0.0, 1.6), //.looking_to(Vec3::X, Vec3::Z),
            Visibility::default(),
            children![
                (
                    WorldModelCamera,
                    Camera3d::default(),
                    Projection::from(PerspectiveProjection {
                        fov: 90.0_f32.to_radians(),
                        ..default()
                    }),
                ),
                // Spawn view model camera.
                (
                    Camera3d::default(),
                    Camera {
                        // Bump the order to render on top of the world model.
                        order: 1,
                        ..default()
                    },
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

    commands
        .spawn((
            Player { movespeed: 2.5 },
            Transform::default(),
            Visibility::default(),
        ))
        .add_child(camera);
}

pub fn move_player(
    time: Res<Time>,
    inputs: Res<ButtonInput<KeyCode>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &Player), (With<Player>, Without<PlayerCamera>)>,
    camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    settings: Res<Settings>,
) {
    let (mut player_transform, player) = player.into_inner();
    let mut camera_transform = camera.into_inner();

    let delta = accumulated_mouse_motion.delta;

    let mut movedir = Vec2::default();

    movedir.y += inputs
        .pressed(settings.inputs.forward)
        .then_some(1.0)
        .unwrap_or_default();
    movedir.y -= inputs
        .pressed(settings.inputs.back)
        .then_some(1.0)
        .unwrap_or_default();
    movedir.x += inputs
        .pressed(settings.inputs.right)
        .then_some(1.0)
        .unwrap_or_default();
    movedir.x -= inputs
        .pressed(settings.inputs.left)
        .then_some(1.0)
        .unwrap_or_default();

    if movedir != Vec2::ZERO {
        movedir = movedir.normalize();

        movedir *= player.movespeed * time.delta_secs();

        let moveby = player_transform.rotation * Vec3::from((movedir, 0.0));
        player_transform.translation += moveby;
    }

    if delta.x != 0.0 {
        let delta_yaw = -delta.x * settings.camera_sensitivity * 0.01;

        player_transform.rotate_local_z(delta_yaw);
    }
    if delta.y != 0.0 {
        let delta_pitch = -delta.y * settings.camera_sensitivity * 0.01;

        const PITCH_LIMIT: f32 = PI - 0.01;
        camera_transform.rotation = Quat::from_rotation_x(
            (camera_transform.rotation.to_euler(EulerRot::XYZ).0 + delta_pitch)
                .clamp(0.01, PITCH_LIMIT),
        );
    }
}
