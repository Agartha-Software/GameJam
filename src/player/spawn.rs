use std::f32::consts::PI;

use bevy::anti_alias::fxaa::Fxaa;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::post_process::dof::DepthOfField;
use bevy::post_process::effect_stack::ChromaticAberration;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

use avian3d::prelude::{LayerMask, LinearVelocity, RayCaster, SpatialQueryFilter};

use crate::player::flashlight::Flashlight;
use crate::player::movement::FLOOR_RAY_PRE_LEN;
use crate::player::{PLAYER_FLOOR_LAYER, Player, PlayerCamera, WorldModelCamera};

pub fn spawn_player(mut commands: Commands) {
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
                        far: 1500.0,
                        ..default()
                    }),
                    Msaa::Off,
                    Fxaa::default(),
                    Bloom::OLD_SCHOOL,
                    ChromaticAberration {
                        intensity: 0.02,
                        max_samples: 6,
                        ..Default::default()
                    },
                    DepthPrepass,
                    DepthOfField {
                        focal_distance: 1.,
                        aperture_f_stops: 2.,
                        ..Default::default()
                    },
                    Tonemapping::TonyMcMapface,
                    DistanceFog {
                        color: Color::srgb_u8(3, 3, 3),
                        falloff: FogFalloff::Exponential { density: 0.25 },
                        ..default()
                    },
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
                    Transform::from_xyz(-0.2, 0.2, 0.5)
                        .looking_to(Vec3::new(0., -0.05, -1.), Vec3::Z),
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
            Player {
                movespeed: 2.5,
                ..default()
            },
            playercast,
            avian3d::dynamics::prelude::RigidBody::Kinematic,
            LinearVelocity::default(),
            Transform::from_xyz(-62.0, -81.0, 22.0),
            Visibility::default(),
        ))
        .add_child(camera);
}
