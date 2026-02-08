use std::ops::{Add, Sub};

use bevy::prelude::*;

use crate::{
    player::{Player, PlayerCamera, WorldModelCamera},
    ui::Cursor,
};

pub struct MarkerPlugin;

impl Plugin for MarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_marker_gltf)
            .add_systems(Update, (spawn_marker, grab_pickup));
    }
}

#[derive(Resource)]
pub struct MarkerAssets {
    pub model: Handle<Gltf>,
    pub material_light: Handle<StandardMaterial>,
}

fn grab_pickup(
    mut cursor_icon: Single<&mut Visibility, With<Cursor>>,
    camera: Single<&GlobalTransform, With<PlayerCamera>>,
    spawned_pickup: Query<&GlobalTransform, With<Pickup>>,
) {
    let mut visible = false;

    for pickup in spawned_pickup {
        if camera.translation().sub(pickup.translation()).length() < 2.0
            && pickup
                .translation()
                .add(Vec3::new(0., 0., 0.5))
                .sub(camera.translation())
                .normalize()
                .dot(*camera.forward())
                > 0.8
        {
            visible = true;
        }
    }

    if visible {
        **cursor_icon = Visibility::Visible;
    } else {
        **cursor_icon = Visibility::Hidden;
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

#[derive(Component)]
struct Pickup;

pub fn spawn_marker(
    mut commands: Commands,
    marker_assets: Res<MarkerAssets>,
    gltf: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut loaded: Local<bool>,
) {
    if *loaded {
        return;
    }

    let Some(gltf) = gltf.get(&marker_assets.model) else {
        return;
    };

    *loaded = true;

    let Some(light) = gltf.named_materials.get("light") else {
        return;
    };

    *materials.get_mut(light).unwrap() = materials
        .get(&marker_assets.material_light)
        .unwrap()
        .clone();

    commands
        .spawn((
            SceneRoot(gltf.scenes[0].clone()),
            Pickup,
            Transform::from_xyz(-62.0, -84.0, 22.3).with_scale(Vec3::splat(1.1)),
        ))
        .with_children(|parent| {
            parent.spawn((
                PointLight {
                    color: Color::srgb(1., 0.5, 0.5),
                    ..Default::default()
                },
                Transform::from_xyz(0.0, 0.0, 0.5),
            ));
        });
}
