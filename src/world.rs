use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{light::NotShadowCaster, prelude::*};
use bevy_atmosphere::prelude::*;
use bevy_sprite3d::prelude::*;

use crate::node::{OilAsset, load_oil, spawn_world_nodes};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Sprite3dPlugin)
            .add_systems(
                Startup,
                (
                    load_ground_gltf,
                    load_ladder,
                    setup_world,
                    play_background_audio,
                    load_oil,
                ),
            )
            .add_systems(Update, (spawn_ground, spawn_world, spawn_world_nodes))
            .insert_resource(Ladder::default())
            .insert_resource(OilAsset::default());
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
pub struct Ladder(Handle<Image>);

fn load_ladder(asset_server: Res<AssetServer>, mut ladder: ResMut<Ladder>) {
    ladder.0 = asset_server.load("ladder.png");
}

fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("ambience.wav")),
        PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::Linear(0.06)),
    ));
}

#[derive(Component)]
pub(crate) struct LadderInteract;

fn spawn_world(
    asset_server: Res<AssetServer>,
    ladder: Res<Ladder>,
    mut meshes: ResMut<Assets<Mesh>>,
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
        Transform::from_xyz(-72.0, -85.0, 25.)
            .looking_to(Vec3::Y, Vec3::Z)
            .with_scale(Vec3::splat(3.)),
    ));

    ladder_parent.with_child((
        Sprite3d {
            pixels_per_metre: 400.,
            alpha_mode: AlphaMode::Blend,
            unlit: false,
            // pivot: Some(Vec2::new(0.5, 0.5)),
            ..default()
        },
        Sprite {
            image: ladder.0.clone(),
            ..default()
        },
        Visibility::Visible,
        LadderInteract,
        Transform::from_xyz(0., 0., 0.),
    ));
    for i in 1..72 {
        ladder_parent.with_child((
            Sprite3d {
                pixels_per_metre: 400.,
                alpha_mode: AlphaMode::Blend,
                unlit: false,
                // pivot: Some(Vec2::new(0.5, 0.5)),
                ..default()
            },
            Sprite {
                image: ladder.0.clone(),
                ..default()
            },
            NotShadowCaster,
            Visibility::Visible,
            Transform::from_xyz(0., i as f32 * 2.24, 0.),
        ));
    }

    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default().mesh().latitudes(7).longitudes(7))),
        MeshMaterial3d(materials.add(Color::srgb_u8(165, 42, 42))),
        Transform::from_xyz(-72.0, -85.0, 500.0).with_scale(Vec3::splat(14.)),
        NotShadowCaster,
    ));
    let mut light_mat = StandardMaterial::default();
    light_mat.base_color = Color::linear_rgb(12.0, 12.0, 5.0);
    light_mat.emissive = LinearRgba::rgb(120.0, 120.0, 50.0);
    light_mat.fog_enabled = false;

    let light_mesh = meshes.add(Sphere::default().mesh().uv(12, 8));

    commands.spawn((
        SpotLight {
            color: Color::srgb_u8(255, 252, 220),
            intensity: 100000000000_f32,
            range: 500.0,
            radius: 0.5,
            shadows_enabled: true,
            outer_angle: PI / 2.0 * 0.01,
            ..Default::default()
        },
        Transform::from_xyz(-72.0, -85.0, 493.0).looking_to(-Vec3::Z, Vec3::Z),
    ));
    commands.spawn((
        Mesh3d(light_mesh.clone()),
        MeshMaterial3d(materials.add(light_mat.clone())),
        NotShadowCaster,
        Transform::from_xyz(-72.0, -91.0, 493.0).with_scale(Vec3::splat(2.)),
    ));
    commands.spawn((
        Mesh3d(light_mesh.clone()),
        MeshMaterial3d(materials.add(light_mat.clone())),
        NotShadowCaster,
        Transform::from_xyz(-69.0, -80.0, 493.0).with_scale(Vec3::splat(2.)),
    ));
    commands.spawn((
        Mesh3d(light_mesh),
        MeshMaterial3d(materials.add(light_mat)),
        NotShadowCaster,
        Transform::from_xyz(-75.0, -80.0, 493.0).with_scale(Vec3::splat(2.)),
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
                    NotShadowCaster,
                    RigidBody::Static,
                    CollisionMargin(0.1),
                ));
            }
        }
    }
}
