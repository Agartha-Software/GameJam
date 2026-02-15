use bevy::prelude::*;

use crate::ui::Cursor;

#[derive(Component)]
pub struct Flashlight;

pub fn control_flashlight(
    mouse: Res<ButtonInput<KeyCode>>,
    mut flashlight: Single<&mut Visibility, With<Flashlight>>,
    mut hand_ui: Single<&mut ImageNode, With<Cursor>>,
) {
    if mouse.just_pressed(KeyCode::KeyF) {
        if **flashlight == Visibility::Hidden {
            **flashlight = Visibility::Visible;
            hand_ui.color = Color::WHITE;
        } else {
            **flashlight = Visibility::Hidden;
            hand_ui.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}
