use bevy::prelude::*;
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
        system::{Commands, ResMut},
    },
    light::NotShadowCaster,
    math::{Dir3, Vec3, primitives::Cuboid},
    mesh::{Mesh, Mesh3d},
    pbr::{DistanceFog, FogFalloff, MeshMaterial3d, StandardMaterial},
    post_process::bloom::Bloom,
    transform::components::Transform,
    utils::default,
};
use bevy_atmosphere::prelude::Gradient;
use bevy_atmosphere::{model::AtmosphereModel, plugin::AtmosphereCamera};

use avian3d::prelude::{LayerMask, LinearVelocity, RayCaster, SpatialQueryFilter};

use crate::player::movement::FLOOR_RAY_PRE_LEN;
use crate::player::{
    PLAYER_FLOOR_LAYER, Player, PlayerCamera, VIEW_MODEL_RENDER_LAYER, WorldModelCamera,
};

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
                    Camera {
                        clear_color: ClearColorConfig::Custom(Color::srgb_u8(0, 0, 0)),
                        // Bump the order to render on top of the world model.
                        order: 1,
                        ..default()
                    },
                    Bloom::OLD_SCHOOL,
                    IsDefaultUiCamera,
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
