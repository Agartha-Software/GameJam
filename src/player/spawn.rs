use std::f32::consts::PI;

use bevy::camera::visibility::RenderLayers;
use bevy::color::palettes::tailwind;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::light::NotShadowCaster;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

use avian3d::prelude::{LayerMask, LinearVelocity, RayCaster, SpatialQueryFilter};

use crate::player::flashlight::Flashlight;
use crate::player::movement::FLOOR_RAY_PRE_LEN;
use crate::player::{
    DEFAULT_RENDER_LAYER, PLAYER_FLOOR_LAYER, Player, PlayerCamera, VIEW_MODEL_RENDER_LAYER,
    WorldModelCamera,
};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::ORANGE_950));

    let camera = commands
        .spawn((
            PlayerCamera,
            Transform::from_xyz(0.0, 0.0, 1.6), //.looking_to(Vec3::X, Vec3::Z),
            Visibility::Visible,
            children![
                (
                    WorldModelCamera,
                    Camera3d::default(),
                    AtmosphereCamera::default(),
                    Camera::default(),
                    Projection::from(PerspectiveProjection {
                        fov: 80.0_f32.to_radians(),
                        far: 2000.0,
                        ..default()
                    }),
                    Bloom::OLD_SCHOOL,
                    Tonemapping::TonyMcMapface,
                    DistanceFog {
                        color: Color::srgb_u8(3, 3, 3),
                        falloff: FogFalloff::Exponential { density: 0.20 },
                        ..default()
                    },
                ),
                // Spawn view model camera.
                (
                    Camera3d::default(),
                    Camera {
                        // Bump the order to render on top of the world model.
                        order: 1,
                        ..default()
                    },
                    Bloom::OLD_SCHOOL,
                    IsDefaultUiCamera,
                    Tonemapping::TonyMcMapface,
                    Projection::from(PerspectiveProjection {
                        fov: 80.0_f32.to_radians(),
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
                (
                    SpotLight {
                        color: Color::WHITE,
                        intensity: 3000000.0,
                        range: 30.0,
                        shadows_enabled: true,
                        radius: 0.1,
                        outer_angle: PI / 2.0 * 0.35,
                        inner_angle: 0.,
                        ..default()
                    },
                    Flashlight,
                    RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER]),
                    Transform::from_xyz(0., 0., 0.5).looking_to(Vec3::new(0., 0.05, -1.), Vec3::Z),
                )
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
            Transform::from_xyz(-62.0, -81.0, 22.0),
            Visibility::default(),
        ))
        .add_child(camera);
}
