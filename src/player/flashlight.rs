use bevy::prelude::*;

use crate::player::WorldModelCamera;

#[derive(Component)]
pub struct Flashlight;

pub fn control_flashlight(
    mouse: Res<ButtonInput<MouseButton>>,
    mut cam: Single<&mut DistanceFog, With<WorldModelCamera>>,

    mut flashlight: Single<&mut Visibility, With<Flashlight>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if **flashlight == Visibility::Hidden {
            **flashlight = Visibility::Visible;
            cam.falloff = FogFalloff::Exponential { density: 0.2 };
        } else {
            **flashlight = Visibility::Hidden;
            cam.falloff = FogFalloff::Exponential { density: 0.5 };
        }
    }
}
