pub mod debug;
pub mod particle;
pub mod player;
mod settings;

use avian3d::{
    PhysicsPlugins,
    prelude::{Collider, CollisionLayers},
};
use bevy::{camera::visibility::RenderLayers, color::palettes::tailwind, prelude::*};
use bevy_atmosphere::plugin::AtmospherePlugin;
use particle::ParticlePlugin;

use crate::{
    debug::DebugPlugin,
    player::{
        DEFAULT_RENDER_LAYER, PLAYER_FLOOR_LAYER, VIEW_MODEL_RENDER_LAYER, move_player,
        spawn_player,
    },
    settings::Settings,
};

fn main() {
    App::new()
        .init_resource::<Settings>()
        .add_plugins((
            DefaultPlugins,
            AtmospherePlugin,
            ParticlePlugin,
            PhysicsPlugins::default(),
            DebugPlugin,
        ))
        .add_systems(
            Startup,
            (spawn_player, spawn_world_model, spawn_lights, spawn_text),
        )
        .add_systems(Update, move_player)
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

fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        })
        .with_child(Text::new(concat!(
            "Move the camera with your mouse.\n",
            "Press arrow up to decrease the FOV of the world model.\n",
            "Press arrow down to increase the FOV of the world model."
        )));
}
