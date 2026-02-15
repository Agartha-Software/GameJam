use std::process::exit;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    node::{OilAsset, OilNode, OilNodeResource},
    player::PlayerCamera,
};

pub struct MarkerPlugin;

impl Plugin for MarkerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<TryPlaceMarker>()
            .add_systems(Startup, load_marker_gltf)
            .add_systems(
                PostUpdate,
                place_markers.run_if(|oil: Res<OilAsset>| oil.loaded),
            );
    }
}

/// distance from the oil node in which the player may place a marker
const PLAYER_PLACE_DIST: f32 = 8.0;

#[derive(Resource)]
struct MarkerAssets {
    pub model: Handle<Gltf>,
    pub material_light: Handle<StandardMaterial>,
}

#[derive(Message)]
pub struct TryPlaceMarker;

fn place_markers(
    mut reader: MessageReader<TryPlaceMarker>,
    mut commands: Commands,
    marker_assets: Res<MarkerAssets>,
    gltf: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut oil_node_res: ResMut<OilNodeResource>,
    spatial_query: SpatialQuery,
    cam: Single<&GlobalTransform, With<PlayerCamera>>,
    nodes: Query<(Entity, &GlobalTransform), With<OilNode>>,
) {
    for TryPlaceMarker in reader.read() {
        let Some(gltf) = gltf.get(&marker_assets.model) else {
            return;
        };

        let Some(light) = gltf.named_materials.get("light") else {
            return;
        };

        *materials.get_mut(light).unwrap() = materials
            .get(&marker_assets.material_light)
            .unwrap()
            .clone();

        if let Some(hits) = spatial_query.cast_ray(
            cam.translation(),
            cam.forward(),
            4.0,
            false,
            &SpatialQueryFilter::default(),
        ) {
            let contact_pos = cam.translation() + hits.distance * cam.forward();

            for (id, transform) in nodes {
                if contact_pos.distance(transform.translation()) < PLAYER_PLACE_DIST {
                    oil_node_res.nodes_left -= 1;

                    commands.entity(id).despawn();

                    commands
                        .spawn((
                            SceneRoot(gltf.scenes[0].clone()),
                            Transform::from_translation(contact_pos),
                            Visibility::default(),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                PointLight {
                                    color: Color::srgb(1., 0.5, 0.5),
                                    intensity: 5000.0,
                                    ..Default::default()
                                },
                                Transform::from_xyz(0.0, 0.0, 0.8),
                            ));
                        });

                    if oil_node_res.nodes_left == 0 {
                        exit(0);
                    }
                }
            }
        }
    }
}

pub fn load_marker_gltf(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let model = assets.load::<Gltf>("marker.glb");

    let mut material_light = StandardMaterial::default();
    material_light.base_color = Color::linear_rgb(10.0, 3.0, 3.0);
    material_light.emissive = LinearRgba::rgb(10.0, 3.0, 3.0);
    let material_light = materials.add(material_light);

    commands.insert_resource(MarkerAssets {
        model,
        material_light,
    });
}
