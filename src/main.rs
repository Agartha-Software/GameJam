pub mod debug;
pub mod particle;
pub mod player;
pub mod settings;
pub mod ui;

use avian3d::{
    PhysicsPlugins,
    prelude::{Collider, CollisionLayers},
};
use bevy::{camera::visibility::RenderLayers, color::palettes::tailwind, prelude::*};
use bevy_aspect_ratio_mask::{AspectRatioMask, AspectRatioPlugin, Resolution};
use bevy_atmosphere::plugin::AtmospherePlugin;
use particle::ParticlePlugin;

use crate::{
    debug::DebugPlugin,
    player::{DEFAULT_RENDER_LAYER, PLAYER_FLOOR_LAYER, PlayerPlugin, VIEW_MODEL_RENDER_LAYER},
    settings::Settings,
    ui::UiPlugin,
};

fn main() {
    App::new()
        .init_resource::<Settings>()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Aspect Ratio Mask".into(),
                        name: Some("Aspect Ratio Mask".into()),
                        ..default()
                    }),
                    ..default()
                }),
            AspectRatioPlugin {
                resolution: Resolution {
                    width: 16.0,
                    height: 9.0,
                },
                mask: AspectRatioMask {
                    color: Color::BLACK,
                },
            },
            AtmospherePlugin,
            ParticlePlugin,
            PhysicsPlugins::default(),
            DebugPlugin,
            UiPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, (spawn_world_model, spawn_lights))
        .run();
}

fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(10.0)));
    let cube = meshes.add(Cuboid::new(2.0, 1.0, 0.5));
    let material_emissive = materials.add(StandardMaterial {
        emissive: LinearRgba::rgb(1000.0, 1000.0, 1000.0),
        ..default()
    });

    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    // The world model camera will render the floor and the cubes spawned in this system.
    // Assigning no `RenderLayers` component defaults to layer 0.

    commands.spawn((
        Mesh3d(floor.clone()),
        MeshMaterial3d(material.clone()),
        Collider::cuboid(20.0, 20.0, 0.1),
        CollisionLayers::new(PLAYER_FLOOR_LAYER, 0),
    ));
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, -3.0, 0.25),
    ));

    // commands.spawn((
    //     Mesh3d(cube),
    //     MeshMaterial3d(material_emissive),
    //     Transform::from_xyz(0.75, 0.0, 1.75),
    // ));
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, -0.75, 4.0),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}
