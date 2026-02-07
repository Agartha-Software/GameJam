use bevy::prelude::*;

#[derive(Component)]
pub struct Flashlight;

pub fn control_flashlight(
    mouse: Res<ButtonInput<MouseButton>>,
    mut flashlight: Single<&mut Visibility, With<Flashlight>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if **flashlight == Visibility::Hidden {
            **flashlight = Visibility::Visible;
        } else {
            **flashlight = Visibility::Hidden;
        }
    }
}
