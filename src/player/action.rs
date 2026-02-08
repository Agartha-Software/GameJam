use std::ops::{Add, Sub};

use bevy::prelude::*;

use crate::{
    marker::MarkerAssets,
    player::{Player, PlayerAction, PlayerCamera, flashlight, marker::Pickup},
    speaker::{Speaker, SpeakerMode, SpeakerResource, grab, ungrab},
    ui,
};

/// distance from the speaker in which the player may place a marker
const PLAYER_SPEAKER_PLACE_DIST: f32 = 4.0;
/// squared distance for easy compute
const PLAYER_SPEAKER_PLACE_DIST_2: f32 = PLAYER_SPEAKER_PLACE_DIST * PLAYER_SPEAKER_PLACE_DIST;

pub fn player_place_marker(
    commands: &mut Commands,
    node: Entity,
    player_transform: &GlobalTransform,
    marker_model: &Handle<Scene>,
) {
    let origin = player_transform.transform_point((0.0, 0.5, 0.0).into());

    commands.entity(node).despawn();

    commands.spawn((
        SceneRoot(marker_model.clone()),
        Transform::from_translation(origin),
        Visibility::default(),
    ));
}

pub fn player_action(
    mut commands: Commands,
    player: Single<(Entity, &GlobalTransform, &mut Player)>,
    speaker: Single<&GlobalTransform, With<Speaker>>,
    speaker_resource: Res<SpeakerResource>,
    input: Res<ButtonInput<MouseButton>>,
    marker_assets: Res<MarkerAssets>,

    mut cursor_icon: Single<&mut Visibility, With<ui::Cursor>>,
    // mut player: Single<(Entity, &mut Player), Without<PlayerCamera>>,
    camera: Single<(Entity, &GlobalTransform), (With<PlayerCamera>, Without<Player>)>,
    mut pickups: Query<(Entity, &GlobalTransform, &mut Transform), With<Pickup>>,
) {
    let (player_entity, player_tm, mut player) = player.into_inner();

    let (camera_entity, camera_tm) = camera.into_inner();

    let lmb = input.just_pressed(MouseButton::Left);

    match player.action {
        PlayerAction::None => {
            let pointed = pickups.iter_mut().find(|(_, global, _)| {
                camera_tm.translation().sub(global.translation()).length() < 2.0
                    && global
                        .translation()
                        .add(Vec3::new(0., 0., 0.5))
                        .sub(camera_tm.translation())
                        .normalize()
                        .dot(*camera_tm.forward())
                        > 0.8
            });
            if let Some((entity, _, mut tm)) = pointed {
                **cursor_icon = Visibility::Visible;

                if lmb {
                    player.action = PlayerAction::HoldingSpeaker(entity.clone());
                    grab(&mut commands, entity, &mut tm, camera_entity);
                }
            } else {
                **cursor_icon = Visibility::Hidden;

                if let SpeakerMode::Ready(node) = &speaker_resource.mode {
                    if player_tm
                        .translation()
                        .distance_squared(speaker.translation())
                        < PLAYER_SPEAKER_PLACE_DIST_2
                        && player.action == PlayerAction::None
                        && input.just_pressed(MouseButton::Left)
                    {
                        player_place_marker(
                            &mut commands,
                            node.clone(),
                            &player_tm,
                            &marker_assets.model,
                        );
                    }
                } else {
                }
            }
        }
        PlayerAction::HoldingSpeaker(entity) => {
            if lmb {
                player.action = PlayerAction::None;
                if let Ok((_, _global, mut tm)) = pickups.get_mut(entity) {
                    ungrab(&mut commands, entity, &mut tm, &player_tm, camera_entity);
                }
            }
        }
        PlayerAction::Dead => {}
    };
}
