use avian3d::prelude::*;
use bevy::prelude::*;

use crate::player::PLAYER_FLOOR_LAYER;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_ground_gltf)
            .add_systems(Update, spawn_ground);
    }
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
