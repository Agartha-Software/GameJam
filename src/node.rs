use bevy::{light::NotShadowCaster, prelude::*};
use bevy_sprite3d::Sprite3d;

#[derive(Component)]
pub struct OilNode;

#[derive(Resource)]
pub struct OilNodeResource {
    pub nodes_left: usize,
}

#[derive(Resource, Default)]
pub struct OilAsset {
    asset: Handle<Image>,
    pub loaded: bool,
}

pub fn load_oil(asset_server: Res<AssetServer>, mut oil: ResMut<OilAsset>) {
    oil.asset = asset_server.load("oilspil.png");
}

pub fn spawn_world_nodes(
    mut commands: Commands,
    mut oil: ResMut<OilAsset>,
    asset_server: Res<AssetServer>,
) {
    if oil.loaded {
        return;
    }

    if !asset_server
        .get_load_state(oil.asset.id())
        .is_some_and(|s| s.is_loaded())
    {
        return;
    }
    oil.loaded = true;

    let node_transforms = [
        Transform::from_xyz(-62.5, -80.9, 22.16).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.0783,
            0.068,
            0.,
        )),
        Transform::from_xyz(-130., 74., 52.9).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.3,
            -0.03,
            0.,
        )),
        Transform::from_xyz(-186.7, -111., 53.6).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -0.34,
            -0.06,
            0.,
        )),
        Transform::from_xyz(173.4, 46.4, 16.7).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.19,
            -0.07,
            0.,
        )),
    ];

    commands.insert_resource(OilNodeResource {
        nodes_left: node_transforms.len(),
    });

    for (idx, transform) in node_transforms.iter().enumerate() {
        commands.spawn((
            Name::new(format!("OilNode-{idx}")),
            Sprite3d {
                pixels_per_metre: 400.,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                // pivot: Some(Vec2::new(0.5, 0.5)),
                ..default()
            },
            Sprite {
                image: oil.asset.clone(),
                ..default()
            },
            NotShadowCaster,
            Visibility::Visible,
            OilNode,
            transform.with_scale(Vec3::splat(40.)),
        ));
    }
}
