use bevy::prelude::*;

use crate::{player::Player, ui::Cursor};

#[derive(Component)]
pub struct Flashlight;

pub fn control_flashlight(
    mouse: Res<ButtonInput<KeyCode>>,
    mut flashlight: Single<&mut Visibility, With<Flashlight>>,
    mut hand_ui: Single<&mut ImageNode, With<Cursor>>,
    mut player: Single<&mut Player>,
) {
    if mouse.just_pressed(KeyCode::KeyF) {
        if **flashlight == Visibility::Hidden {
            player.light = true;
            **flashlight = Visibility::Visible;
            hand_ui.color = Color::WHITE;
        } else {
            player.light = false;
            **flashlight = Visibility::Hidden;
            hand_ui.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}
