use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

use crate::player::PLAYER_FLOOR_LAYER;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_ground_gltf, setup_world))
            .add_systems(Update, spawn_ground);
    }
}

pub fn setup_world(mut ambient: ResMut<GlobalAmbientLight>, mut commands: Commands) {
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
pub struct Ground;

pub fn load_ground_gltf(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(assets.load::<Scene>(GltfAssetLabel::Scene(0).from_asset("terrain.glb"))),
        Ground,
    ));
}

pub fn spawn_ground(
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
