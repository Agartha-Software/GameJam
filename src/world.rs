use bevy::prelude::*;

pub fn spawn_world(mut commands: Commands, assets: Res<AssetServer>) {
    let ground = assets.load::<Scene>(GltfAssetLabel::Scene(0).from_asset("terrain.glb"));

    commands.spawn((
        SceneRoot(ground),
        //avian3d::dynamics::prelude::RigidBody::Static,
    ));
}
