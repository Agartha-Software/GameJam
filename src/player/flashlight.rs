use bevy::prelude::*;

use crate::{player::WorldModelCamera, ui::Cursor};

#[derive(Component)]
pub struct Flashlight;

pub fn control_flashlight(
    mouse: Res<ButtonInput<MouseButton>>,
    mut cam: Single<&mut DistanceFog, With<WorldModelCamera>>,

    mut flashlight: Single<&mut Visibility, With<Flashlight>>,
    mut hand_ui: Single<&mut ImageNode, With<Cursor>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if **flashlight == Visibility::Hidden {
            **flashlight = Visibility::Visible;
            cam.falloff = FogFalloff::Exponential { density: 0.2 };
            hand_ui.color = Color::WHITE;
        } else {
            **flashlight = Visibility::Hidden;
            cam.falloff = FogFalloff::Exponential { density: 0.3 };
            hand_ui.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}
