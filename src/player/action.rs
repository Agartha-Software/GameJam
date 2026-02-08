use std::{
    ops::{Add, Sub},
    process::exit,
};

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    marker::MarkerAssets,
    player::{Player, PlayerAction, PlayerCamera, marker::Pickup},
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

#[derive(Resource, Default)]
pub struct Placed(u32);

pub fn player_action(
    mut commands: Commands,
    time: Res<Time>,
    player: Single<
        (
            Entity,
            &GlobalTransform,
            &mut Transform,
            &mut Player,
            &RayHits,
        ),
        (Without<PlayerCamera>, Without<Pickup>),
    >,
    speaker: Single<&GlobalTransform, With<Speaker>>,
    speaker_resource: Res<SpeakerResource>,
    input: Res<ButtonInput<MouseButton>>,
    marker_assets: Res<MarkerAssets>,

    mut cursor_icon: Single<&mut Visibility, With<ui::Cursor>>,
    camera: Single<
        (Entity, &GlobalTransform, &mut Transform),
        (With<PlayerCamera>, Without<Pickup>, Without<Player>),
    >,
    mut pickups: Query<
        (Entity, &GlobalTransform, &mut Transform),
        (With<Pickup>, Without<Player>, Without<PlayerCamera>),
    >,
    entities: Query<
        (Entity, &GlobalTransform),
        (Without<Player>, Without<PlayerCamera>, Without<Pickup>),
    >,
    mut placed: Local<Placed>,
) {
    let (player_entity, player_global, mut player_tm, mut player, player_hits) =
        player.into_inner();

    let (camera_entity, camera_global, mut camera_tm) = camera.into_inner();

    let lmb = input.just_pressed(MouseButton::Left);

    let next_action = match &player.action {
        PlayerAction::None => {
            let pointed = pickups.iter_mut().find(|(_, global, _)| {
                camera_global
                    .translation()
                    .sub(global.translation())
                    .length()
                    < 2.0
                    && global
                        .translation()
                        .add(Vec3::new(0., 0., 0.5))
                        .sub(camera_global.translation())
                        .normalize()
                        .dot(*camera_global.forward())
                        > 0.8
            });
            if let Some((entity, _, mut tm)) = pointed {
                if lmb {
                    grab(&mut commands, entity, &mut tm, camera_entity);
                    **cursor_icon = Visibility::Hidden;
                    Some(PlayerAction::HoldingSpeaker(entity.clone()))
                } else {
                    **cursor_icon = Visibility::Visible;
                    None
                }
            } else {
                **cursor_icon = Visibility::Hidden;

                if let SpeakerMode::Ready(node) = &speaker_resource.mode {
                    if player_global
                        .translation()
                        .distance_squared(speaker.translation())
                        < PLAYER_SPEAKER_PLACE_DIST_2
                        && player.action == PlayerAction::None
                        && input.just_pressed(MouseButton::Left)
                        && !player_hits.is_empty()
                    {
                        placed.0 += 1;
                        if placed.0 == 4 {
                            exit(0);
                        }
                        player_place_marker(
                            &mut commands,
                            node.clone(),
                            &player_global,
                            &marker_assets.model,
                        );
                    }
                } else {
                }
                None
            }
        }
        PlayerAction::HoldingSpeaker(entity) => {
            if lmb {
                if let Ok((_, _global, mut tm)) = pickups.get_mut(entity.clone()) {
                    ungrab(
                        &mut commands,
                        entity.clone(),
                        &mut tm,
                        &player_global,
                        camera_entity,
                    );
                }
                Some(PlayerAction::None)
            } else {
                None
            }
        }
        PlayerAction::Dying(timeout, e) => {
            if timeout.gt(&1.0) {
                exit(0);
                Some(PlayerAction::Dead)
            } else {
                if let Ok((_, global)) = entities.get(e.entity()) {
                    let dxy = (global.translation() - player_global.translation()).xy();
                    let z = dxy.angle_to(player_global.up().xy());

                    let dz = (global.translation() - camera_global.translation()).z;

                    let x = (dz / dxy.length()).atan();

                    player_tm.rotation *= Quat::from_rotation_z(-z);
                    camera_tm.rotation = Quat::from_rotation_x(90f32.to_radians() + x);
                }

                Some(PlayerAction::Dying(timeout + time.delta_secs(), e.clone()))
            }
        }
        _ => None,
    };
    if let Some(next_action) = next_action {
        player.action = next_action;
    }
}
