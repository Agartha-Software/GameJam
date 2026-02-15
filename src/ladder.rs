use bevy::{light::NotShadowCaster, prelude::*};
use bevy_sprite3d::Sprite3d;

use crate::{node::OilNodeResource, ui::CenteredText};

pub struct LadderPlugin;

impl Plugin for LadderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Ladder::default())
            .add_systems(Startup, load_ladder)
            .add_systems(Update, (/*ladder_interraction,*/spawn_ladder));
    }
}

// pub fn ladder_interraction(
//     win_text: Query<(&mut Node, &mut Text), With<CenteredText>>,
//     mut oil_res: ResMut<OilNodeResource>,
// ) {
//     if oil_res.nodes_left == 0 {}
// }

#[derive(Resource, Default)]
pub struct Ladder(Handle<Image>);

#[derive(Component)]
pub struct LadderInteract;

fn load_ladder(asset_server: Res<AssetServer>, mut ladder: ResMut<Ladder>) {
    ladder.0 = asset_server.load("ladder.png");
}

fn spawn_ladder(
    asset_server: Res<AssetServer>,
    ladder: Res<Ladder>,
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
}
