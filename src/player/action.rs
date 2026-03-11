use std::{
    ops::{Add, Sub},
};

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    monster::Monster,
    player::{Player, PlayerAction, PlayerCamera, marker::TryPlaceMarker},
    speaker::{Pickup, grab, ungrab},
    ui::{self, DeathScreen, OverlayImage},
};

pub const THROW_VEL: f32 = 2.0;
pub const THROW_RECOIL: f32 = 0.5;

#[derive(Message)]
pub struct PlayerDeath;

pub fn player_death(
    mut ui: Single<&mut ImageNode, With<OverlayImage>>,
    mut screen: Single<&mut Visibility, With<DeathScreen>>,
    mut death: MessageReader<PlayerDeath>,
    mut timeout: Local<Option<f32>>,
    time: Res<Time<Real>>,
    mut exit: MessageWriter<AppExit>,
    spatial_audio: Query<&mut SpatialAudioSink, Without<Monster>>,
    audio: Query<&mut AudioSink>,
) {
    if let Some(_) = death.read().next() {
        timeout.get_or_insert(1.0);
        **screen = Visibility::Visible;
        for s in spatial_audio {
            s.stop();
        }
        for s in audio {
            s.stop();
        }
    }
    if let Some(timeout) = timeout.as_mut() {
        *timeout -= time.delta_secs() / 3.0;
        ui.color.set_alpha(timeout.clamp(0., 1.));
        if *timeout < 0. {
            exit.write(AppExit::Success);
        }
    }
}

pub fn player_action(
    mut commands: Commands,
    time: Res<Time>,
    player: Single<
        (
            &GlobalTransform,
            &mut Transform,
            &mut Player,
            &mut LinearVelocity,
        ),
        (Without<PlayerCamera>, Without<Pickup>),
    >,
    mut place_speaker: MessageWriter<TryPlaceMarker>,
    input: Res<ButtonInput<MouseButton>>,
    mut cursor_icon: Single<&mut Visibility, With<ui::Cursor>>,
    camera: Single<
        (Entity, &GlobalTransform, &mut Transform),
        (With<PlayerCamera>, Without<Pickup>, Without<Player>),
    >,
    mut pickups: Query<
        (
            Entity,
            &GlobalTransform,
            &mut Transform,
            &mut LinearVelocity,
        ),
        (With<Pickup>, Without<Player>, Without<PlayerCamera>),
    >,
    entities: Query<
        (Entity, &GlobalTransform),
        (Without<Player>, Without<PlayerCamera>, Without<Pickup>),
    >,
    mut death: MessageWriter<PlayerDeath>,
) {
    let (player_global, mut player_tm, mut player, mut player_vel) = player.into_inner();

    let (camera_entity, camera_global, mut camera_tm) = camera.into_inner();

    let lmb = input.just_pressed(MouseButton::Left);

    let next_action = match &player.action {
        PlayerAction::None => {
            let pointed = pickups.iter_mut().find(|(_, global, _, _)| {
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
            if let Some((entity, _, mut tm, _vel)) = pointed {
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

                if lmb {
                    place_speaker.write(TryPlaceMarker);
                }
                None
            }
        }
        PlayerAction::HoldingSpeaker(entity) => {
            if lmb {
                if let Ok((_, _global, mut tm, mut vel)) = pickups.get_mut(entity.clone()) {
                    ungrab(
                        &mut commands,
                        entity.clone(),
                        &mut tm,
                        &player_global,
                        camera_entity,
                        &mut player_vel,
                        &mut vel,
                        *camera_global.forward(),
                    );
                }
                Some(PlayerAction::None)
            } else {
                None
            }
        }
        PlayerAction::Dying => {
            death.write(PlayerDeath);
            Some(PlayerAction::Dead)
        }
        _ => None,
    };
    if let Some(next_action) = next_action {
        player.action = next_action;
    }
}
