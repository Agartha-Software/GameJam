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
    pub model: Handle<Gltf>,
    pub material_light: Handle<StandardMaterial>,
}

#[derive(Message)]
pub struct PlaceMarker {
    pub oil: Entity,
}

fn place_markers(
    mut reader: MessageReader<PlaceMarker>,
    mut commands: Commands,
    marker_assets: Res<MarkerAssets>,
    gltf: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player: Single<(&GlobalTransform, &RayHits), With<Player>>,
) {
    let (player_transform, _) = player.into_inner();

    for PlaceMarker { oil } in reader.read() {
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

        let origin = player_transform.transform_point((0.0, 0.5, 0.0).into());

        commands.entity(*oil).despawn();

        commands
            .spawn((
                SceneRoot(gltf.scenes[0].clone()),
                Transform::from_translation(origin),
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
