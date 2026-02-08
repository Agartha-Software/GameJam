use std::f32::consts::PI;

use bevy::{light::NotShadowCaster, prelude::*};
use bevy_sprite3d::Sprite3d;

#[derive(Component)]
pub struct OilNode;

#[derive(Resource)]
pub struct OilNodeResource {
    pub nodes_left: usize,
}

#[derive(Resource, Default)]
pub struct OilAsset(Handle<Image>);

pub fn load_oil(asset_server: Res<AssetServer>, mut oil: ResMut<OilAsset>) {
    oil.0 = asset_server.load("oilspil.png");
}

pub fn spawn_world_nodes(
    mut commands: Commands,
    oil: Res<OilAsset>,
    asset_server: Res<AssetServer>,
    mut loaded: Local<bool>,
) {
    if *loaded {
        return;
    }

    if !asset_server
        .get_load_state(oil.0.id())
        .is_some_and(|s| s.is_loaded())
    {
        return;
    }
    *loaded = true;

    for transform in [
        Transform::from_xyz(-62.0, -81.0, 22.15).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            4.5 * PI / 180.,
            4.5 * PI / 180.,
            0.,
        )),
        Transform::from_xyz(-42.0, -81.0, 22.0),
        Transform::from_xyz(-42.0, -61.0, 22.0),
        Transform::from_xyz(0.0, 0.0, 22.0),
    ] {
        commands.spawn((
            Sprite3d {
                pixels_per_metre: 400.,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                // pivot: Some(Vec2::new(0.5, 0.5)),
                ..default()
            },
            Sprite {
                image: oil.0.clone(),
                ..default()
            },
            NotShadowCaster,
            Visibility::Visible,
            OilNode,
            transform.with_scale(Vec3::splat(20.)),
        ));
    }
}
