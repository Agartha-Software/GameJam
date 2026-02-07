use avian3d::{math::PI, prelude::*};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{
    player::{Player, PlayerCamera},
    settings::{self, Settings},
};

/// Acceleration in m/s^2
pub const PLAYER_ACCELERATION: f32 = 40.0 / 3.6;

/// Velocity in m/s calculated from km/h
pub const PLAYER_MAX_SPEED: f32 = 4.0 / 3.6;
/// Velocity squared to optimize comparaisons
pub const PLAYER_MAX_SPEED_2: f32 = PLAYER_MAX_SPEED * PLAYER_MAX_SPEED;

// Effective gravity in m/s^2 in Z
pub const PLAYER_BUOYANCY: f32 = -5.;

// Jump velocity
pub const PLAYER_JUMP_IMPULSE: f32 = 0.5;

pub const FLOOR_RAY_PRE_LEN: f32 = 1.0;

fn move_player_camera(
    delta: &Vec2,
    player_transform: &mut Transform,
    camera_transform: &mut Transform,
    sensitivity: f32,
) {
    if delta.x != 0.0 {
        let delta_yaw = -delta.x * sensitivity * 0.01;

        player_transform.rotate_local_z(delta_yaw);
    }
    if delta.y != 0.0 {
        let delta_pitch = -delta.y * sensitivity * 0.01;

        const PITCH_LIMIT: f32 = PI - 0.01;
        camera_transform.rotation = Quat::from_rotation_x(
            (camera_transform.rotation.to_euler(EulerRot::XYZ).0 + delta_pitch)
                .clamp(0.01, PITCH_LIMIT),
        );
    }
}

fn get_wishdir(inputs: &ButtonInput<KeyCode>, keys: &settings::Inputs) -> Vec2 {
    let mut wishdir = Vec2::default();

    wishdir.y += inputs
        .pressed(keys.forward)
        .then_some(1.0)
        .unwrap_or_default();
    wishdir.y -= inputs.pressed(keys.back).then_some(1.0).unwrap_or_default();
    wishdir.x += inputs
        .pressed(keys.right)
        .then_some(1.0)
        .unwrap_or_default();
    wishdir.x -= inputs.pressed(keys.left).then_some(1.0).unwrap_or_default();
    wishdir
}

pub fn move_realistic(
    mut wishdir: Vec2,
    floor_hits: &RayHits,
    mut velocity: Mut<'_, LinearVelocity>,
    mut transform: Mut<'_, Transform>,
    settings: Res<Settings>,
    inputs: Res<ButtonInput<KeyCode>>,
    player: &Player,
    time: Res<Time>,
) {
    if let Some(ground) = floor_hits.first() {
        if velocity.0.xy().dot(velocity.0.xy())
            > PLAYER_MAX_SPEED_2 * player.movespeed * player.movespeed
            || wishdir == Vec2::ZERO
        {
            wishdir = -velocity.0.xy();
        }

        if velocity.0.z <= 0.0 {
            velocity.0.z = 0.0;
            if inputs.pressed(settings.inputs.jump) {
                velocity.0.z = PLAYER_JUMP_IMPULSE;
            }
        }
        transform.translation.z += (FLOOR_RAY_PRE_LEN - ground.distance) * time.delta_secs();
    } else {
        velocity.0.z += PLAYER_BUOYANCY * time.delta_secs() * time.delta_secs();
    }

    if wishdir != Vec2::ZERO {
        let mut moveforce = wishdir * PLAYER_MAX_SPEED - velocity.0.xy();

        moveforce /= PLAYER_MAX_SPEED;
        moveforce *= PLAYER_ACCELERATION * time.delta_secs() * time.delta_secs();

        if floor_hits.is_empty() {
            moveforce *= 0.1;
        }

        let moveforce = Vec3::from((moveforce, 0.0));

        velocity.0 += moveforce;
        transform.translation += (moveforce * time.delta_secs()) / 2.0;
    }
}

pub fn move_op(
    wishdir: Vec2,
    mut velocity: Mut<'_, LinearVelocity>,
    mut transform: Mut<'_, Transform>,
    settings: Res<Settings>,
    inputs: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    velocity.0 = Vec3::ZERO;

    let mut moveforce = Vec3::from((wishdir, 0.0));

    if inputs.pressed(settings.inputs.jump) {
        moveforce.z = 1.;
    }
    if inputs.pressed(settings.inputs.down) {
        moveforce.z = -1.;
    }

    transform.translation += moveforce * time.delta_secs() * 15.;
}

pub fn move_player(
    time: Res<Time>,
    inputs: Res<ButtonInput<KeyCode>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<
        (&mut Transform, &Player, &RayHits, &mut LinearVelocity),
        (With<Player>, Without<PlayerCamera>),
    >,
    camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    settings: Res<Settings>,
) {
    let (mut player_transform, player, floor_hits, velocity) = player.into_inner();
    let mut camera_transform = camera.into_inner();

    player_transform.translation += velocity.0 * time.delta_secs();

    move_player_camera(
        &accumulated_mouse_motion.delta,
        &mut player_transform,
        &mut camera_transform,
        settings.camera_sensitivity,
    );

    let mut wishdir = get_wishdir(&inputs, &settings.inputs);

    if wishdir != Vec2::ZERO {
        wishdir = wishdir.normalize();
        wishdir = (player_transform.rotation * Vec3::from((wishdir, 0.0))).xy();
    }

    if !settings.debug {
        move_realistic(
            wishdir,
            floor_hits,
            velocity,
            player_transform,
            settings,
            inputs,
            player,
            time,
        )
    } else {
        move_op(wishdir, velocity, player_transform, settings, inputs, time)
    }
}
