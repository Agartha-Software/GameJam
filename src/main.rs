// pub mod world;

// use bevy::{
//     DefaultPlugins, app::Startup, asset::{AssetServer, Assets}, camera::Camera3d, color::Srgba, ecs::{component::Component, query::With, system::{Commands, Query, Res, ResMut}}, input::{ButtonInput, keyboard::KeyCode}, math::{Vec3, primitives::{Plane3d, Sphere}}, mesh::{Mesh, Mesh3d}, pbr::{MeshMaterial3d, StandardMaterial}, prelude::App, transform::components::Transform
// };

// #[derive(Component)]
// struct Player{}

// fn setup_graphics(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {

//     let sphere_mesh = meshes.add(Sphere::new(0.45));
//     let plane_mesh = meshes.add(Plane3d::new((0f32, 0f32, 1f32).into(), (25f32, 25f32).into()));

//     // Add a camera so we can see the debug-render.
//     commands.spawn((
//         Player{},
//         Camera3d::default(),
//         Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
//     ));

//     commands.spawn((
//                 Mesh3d(sphere_mesh.clone()),
//                 MeshMaterial3d(materials.add(StandardMaterial {
//                     base_color: Srgba::hex("#ffd891").unwrap().into(),
//                     // vary key PBR parameters on a grid of spheres to show the effect
//                     ..Default::default()
//                 })),
//                 Transform::from_xyz(0.0, 0.0, 0.0),

//             ));

//     commands.spawn((
//                 Mesh3d(plane_mesh.clone()),
//                 MeshMaterial3d(materials.add(StandardMaterial {
//                     base_color: Srgba::hex("#ff1111").unwrap().into(),
//                     // vary key PBR parameters on a grid of spheres to show the effect
//                     ..Default::default()
//                 })),
//                 Transform::from_xyz(0.0, 0.0, 0.0),
//             ));
// }
pub mod player;
mod settings;

use avian3d::{PhysicsPlugins, prelude::{Collider, CollisionLayers}};
use bevy::{camera::visibility::RenderLayers, color::palettes::tailwind, prelude::*};

use crate::{
    player::{
        DEFAULT_RENDER_LAYER, PLAYER_FLOOR_LAYER, PlayerFloorCast, VIEW_MODEL_RENDER_LAYER, move_player, spawn_player
    },
    settings::Settings,
};

fn main() {
    App::new()
        .init_resource::<Settings>()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
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
    let material = materials.add(Color::WHITE);

    // The world model camera will render the floor and the cubes spawned in this system.
    // Assigning no `RenderLayers` component defaults to layer 0.

    commands.spawn((
        Mesh3d(floor.clone()),
        MeshMaterial3d(material.clone()),
        Collider::cuboid(20.0, 20.0, 0.1),
        CollisionLayers::new(PLAYER_FLOOR_LAYER, 0),
    ));
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -1.0).with_scale((10.0, 10.0, 1.0).into()),
        Mesh3d(floor),
        MeshMaterial3d(material.clone()),
        Collider::cuboid(20.0, 20.0, 0.1),
        CollisionLayers::new(PLAYER_FLOOR_LAYER, 0),
    ));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, -3.0, 0.25),
    ));

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material),
        Transform::from_xyz(0.75, 0.0, 1.75),
    ));
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
