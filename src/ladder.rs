use bevy::{light::NotShadowCaster, prelude::*};
use bevy_sprite3d::Sprite3d;

use crate::{node::OilNodeResource, player::PlayerCamera, ui::CenteredText};

pub struct LadderPlugin;

impl Plugin for LadderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Ladder::default())
            .add_systems(Startup, load_ladder)
            .add_systems(Update, spawn_ladder)
            .add_systems(PostUpdate, game_state)
            .insert_resource(GameState::default());
    }
}

#[derive(Resource, Default)]
pub struct GameState {
    current_step: Step,
    since_last_change: Timer,
}

#[derive(Default, PartialEq)]
pub enum Step {
    #[default]
    LocateOil,
    Exit,
    SorryBuddy,
}

pub fn game_state(
    text: Single<(&mut Node, &mut Text), With<CenteredText>>,
    oil_res: If<Res<OilNodeResource>>,
    mut game_state: ResMut<GameState>,
    ladder: Single<&GlobalTransform, With<LadderInteract>>,
    mut ladder_parent: Single<&mut Transform, With<LadderParent>>,
    player_cam: Single<&GlobalTransform, With<PlayerCamera>>,
    time: Res<Time>,
) {
    if !game_state.since_last_change.is_finished() {
        game_state.since_last_change.tick(time.delta());
    }

    let (mut node, mut text) = text.into_inner();

    if oil_res.nodes_left <= 0 && game_state.current_step == Step::LocateOil {
        game_state.current_step = Step::Exit;
        game_state.since_last_change = Timer::from_seconds(10.0, TimerMode::Once);

        node.display = Display::Block;
        text.0 = "Just get back to the ladder now!".to_string();
    }

    if game_state.current_step == Step::Exit {
        if game_state.since_last_change.just_finished() {
            node.display = Display::None;
        }
        if !game_state.since_last_change.is_finished() {
            ladder_parent.translation += Vec3::Z * 30.0 * time.delta_secs();
        }
    }

    if game_state.current_step == Step::Exit
        && (ladder.translation().xy() - player_cam.translation().xy()).length() <= 7.0
    {
        game_state.current_step = Step::SorryBuddy;

        node.display = Display::Block;
        text.0 = "Sorry buddy... got an order from up there...".to_string();
    }
}

#[derive(Resource, Default)]
pub struct Ladder(Handle<Image>);

#[derive(Component)]
pub struct LadderInteract;

#[derive(Component)]
pub struct LadderParent;

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
        LadderParent,
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
