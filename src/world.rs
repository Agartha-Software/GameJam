use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_sprite3d::prelude::*;

use crate::player::PLAYER_FLOOR_LAYER;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Sprite3dPlugin)
            .add_systems(Startup, (load_ground_gltf, load_ladder, setup_world))
            .add_systems(Update, (spawn_ground, spawn_ladder))
            .insert_resource(Ladder::default());
    }
}

fn setup_world(mut ambient: ResMut<GlobalAmbientLight>, mut commands: Commands) {
    commands.insert_resource(AtmosphereModel::new(bevy_atmosphere::prelude::Gradient {
        sky: Color::srgb_u8(7, 9, 18).into(),
        horizon: Color::srgb_u8(3, 3, 3).into(),
        ground: Color::srgb_u8(5, 5, 5).into(),
        height: 0.7,
    }));

    commands.insert_resource(AtmosphereSettings {
        dithering: false,
        resolution: 1024,
        ..Default::default()
    });

    *ambient = GlobalAmbientLight {
        color: Color::srgb_u8(7, 8, 18).into(),
        brightness: 10000.0,
        affects_lightmapped_meshes: true,
    };
}

#[derive(Component)]
struct Ground;

fn load_ground_gltf(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(assets.load::<Scene>(GltfAssetLabel::Scene(0).from_asset("terrain.glb"))),
        Ground,
    ));
}

#[derive(Resource, Default)]
struct Ladder(Handle<Image>);

fn load_ladder(asset_server: Res<AssetServer>, mut ladder: ResMut<Ladder>) {
    ladder.0 = asset_server.load("ladder.png");
}

fn spawn_ladder(
    asset_server: Res<AssetServer>,
    ladder: Res<Ladder>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut loaded: Local<bool>,
) {
    if *loaded {
        return;
    }

    if !asset_server
        .get_load_state(ladder.0.id())
        .is_some_and(|s| s.is_loaded())
    {
        return;
    }

    *loaded = true;

    let mut ladder_parent = commands.spawn((
        Visibility::Visible,
        Transform::from_xyz(-62.0, -83.0, 25.5)
            .looking_to(Vec3::Y, Vec3::Z)
            .with_scale(Vec3::splat(3.)),
    ));

    for i in 0..72 {
        ladder_parent.with_child((
            Sprite3d {
                pixels_per_metre: 400.,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                // pivot: Some(Vec2::new(0.5, 0.5)),
                ..default()
            },
            Sprite {
                image: ladder.0.clone(),
                ..default()
            },
            Visibility::Visible,
            Transform::from_xyz(0., i as f32 * 2.24, 0.),
        ));
    }

    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default().mesh().latitudes(7).longitudes(7))),
        MeshMaterial3d(materials.add(Color::srgb_u8(165, 42, 42))),
        Transform::from_xyz(-62.0, -83.0, 500.0).with_scale(Vec3::splat(14.)),
    ));
    let mut light_mat = StandardMaterial::default();
    light_mat.base_color = Color::linear_rgb(7.0, 7.0, 5.0);
    light_mat.emissive = LinearRgba::rgb(70.0, 70.0, 50.0);
    light_mat.fog_enabled = false;

    let light_mesh = meshes.add(Sphere::default().mesh().uv(12, 8));

    commands.spawn((
        SpotLight {
            color: Color::srgb_u8(255, 252, 220),
            intensity: 300000000000_f32,
            range: 500.0,
            radius: 0.5,
            shadows_enabled: false,
            outer_angle: PI / 2.0 * 0.016,
            ..Default::default()
        },
        Transform::from_xyz(-62.0, -83.0, 493.0).looking_to(-Vec3::Z, Vec3::Z),
    ));
    commands.spawn((
        Mesh3d(light_mesh.clone()),
        MeshMaterial3d(materials.add(light_mat.clone())),
        Transform::from_xyz(-62.0, -89.0, 493.0).with_scale(Vec3::splat(0.5)),
    ));
    commands.spawn((
        Mesh3d(light_mesh.clone()),
        MeshMaterial3d(materials.add(light_mat.clone())),
        Transform::from_xyz(-59.0, -78.0, 493.0).with_scale(Vec3::splat(0.5)),
    ));
    commands.spawn((
        Mesh3d(light_mesh),
        MeshMaterial3d(materials.add(light_mat)),
        Transform::from_xyz(-65.0, -78.0, 493.0).with_scale(Vec3::splat(0.5)),
    ));
}

fn spawn_ground(
    mut commands: Commands,
    ground_scene: Query<Entity, With<Ground>>,
    children: Query<&Children>,
    mut meshs: Query<Entity, With<Mesh3d>>,
    mut loaded: Local<bool>,
) {
    if *loaded {
        return;
    }

    for ground_scene_entity in &ground_scene {
        for entity in children.iter_descendants(ground_scene_entity) {
            *loaded = true;
            if let Ok(e) = meshs.get_mut(entity) {
                commands.entity(e).insert((
                    ColliderConstructor::TrimeshFromMesh,
                    CollisionLayers::new(PLAYER_FLOOR_LAYER, 0),
                ));
            }
        }
    }
}
