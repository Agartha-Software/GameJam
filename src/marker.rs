use avian3d::prelude::*;
use bevy::prelude::*;

use crate::player::Player;

pub struct MarkerPlugin;

impl Plugin for MarkerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<PlaceMarker>()
            .add_systems(Startup, load_marker_gltf)
            .add_systems(PostUpdate, place_markers);
    }
}

#[derive(Resource)]
struct MarkerAssets {
    pub model: Handle<Scene>,
    // pub material_blink: Option<Handle<StandardMaterial>>,
}

#[derive(Message)]
pub struct PlaceMarker {
    pub oil: Entity,
}

fn place_markers(
    mut reader: MessageReader<PlaceMarker>,
    mut commands: Commands,
    marker_assets: Res<MarkerAssets>,
    player: Single<(&GlobalTransform, &RayHits), With<Player>>,
) {
    let (player_transform, _) = player.into_inner();

    for PlaceMarker { oil } in reader.read() {
        println!("Message received!");
        let origin = player_transform.transform_point((0.0, 0.5, 0.0).into());

        commands.entity(*oil).despawn();

        commands.spawn((
            SceneRoot(marker_assets.model.clone()),
            Transform::from_translation(origin),
            Visibility::default(),
        ));
    }
}

fn load_marker_gltf(mut commands: Commands, assets: Res<AssetServer>) {
    // let model = assets.load::<Gltf>("marker.glb");

    let model = assets.load::<Scene>(GltfAssetLabel::Scene(0).from_asset("marker.glb"));

    commands.insert_resource(MarkerAssets { model });
}

// fn spawn_marker(
//     mut commands: Commands,
//     gltf: Res<Assets<Gltf>>,
//     mut Marker_assets: ResMut<MarkerAssets>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut stop: Local<bool>,
// ) {
//     if *stop {
//         return;
//     }

//     let Some(gltf) = gltf.get(&Marker_assets.model) else {
//         return;
//     };

//     let Some(blink) = gltf.named_materials.get("light") else {
//         return;
//     };

//     let Some(material) = materials.get_mut(blink) else {
//         return;
//     };

//     material.emissive = LinearRgba::rgb(0.0, 4.0, 0.0);

//     Marker_assets.material_blink = Some(blink.clone());

//     *stop = true;

//     commands.spawn((
//         SceneRoot(gltf.scenes[0].clone()),
//         Marker::default(),
//         avian3d::dynamics::prelude::RigidBody::Kinematic,
//         LinearVelocity::default(),
//         Transform::from_xyz(-64.0, -81.0, 22.25),
//     ));
// }
