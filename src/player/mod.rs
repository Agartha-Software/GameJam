pub mod action;
pub mod flashlight;
pub mod marker;
pub mod movement;
pub mod spawn;

use bevy::prelude::*;

use crate::player::action::player_action;
use crate::player::flashlight::control_flashlight;
use crate::player::marker::MarkerPlugin;
use crate::player::movement::move_player;
use crate::player::spawn::spawn_player;

#[derive(Debug, Default, PartialEq)]
pub enum PlayerAction {
    #[default]
    None,
    HoldingSpeaker(Entity),
    // PlacingMarker,
    Dead,
}

#[derive(Debug, Component, Default)]
pub struct Player {
    pub movespeed: f32,
    pub action: PlayerAction,
}

#[derive(Debug, Component)]
pub struct PlayerCamera;

#[derive(Reflect)]
pub struct PlayerFloorCast;

#[derive(Debug, Component)]
pub struct WorldModelCamera;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
pub const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
pub const VIEW_MODEL_RENDER_LAYER: usize = 1;

pub const PLAYER_FLOOR_LAYER: u32 = 2;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MarkerPlugin)
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, control_flashlight, player_action));
    }
}
