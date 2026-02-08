use bevy::prelude::*;

use crate::{
    marker::MarkerAssets,
    player::{Player, PlayerAction},
    speaker::{Speaker, SpeakerMode, SpeakerResource},
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

pub fn player_marker_system(
    mut commands: Commands,
    player: Single<(&GlobalTransform, &Player)>,
    speaker: Single<&GlobalTransform, With<Speaker>>,
    speaker_resource: Res<SpeakerResource>,
    input: Res<ButtonInput<MouseButton>>,
    marker_assets: Res<MarkerAssets>,
) {
    let (player_transform, player) = player.into_inner();

    if let SpeakerMode::Ready(node) = &speaker_resource.mode {
        if player_transform
            .translation()
            .distance_squared(speaker.translation())
            < PLAYER_SPEAKER_PLACE_DIST_2
            && player.action == PlayerAction::None
            && input.just_pressed(MouseButton::Left)
        {
            player_place_marker(
                &mut commands,
                node.clone(),
                &player_transform,
                &marker_assets.model,
            );
        }
    };
}
