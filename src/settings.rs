use bevy::{ecs::resource::Resource, input::keyboard::KeyCode};

#[derive(Resource)]
pub struct Settings {
    pub camera_sensitivity: f32,
    pub inputs: Inputs,
    pub chromatic_aberation: f32,
}

pub struct Inputs {
    pub forward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub back: KeyCode,
    pub jump: KeyCode,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyW,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            back: KeyCode::KeyS,
            jump: KeyCode::Space,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            camera_sensitivity: 1.0,
            inputs: Default::default(),
            chromatic_aberation: 1.2,
        }
    }
}
