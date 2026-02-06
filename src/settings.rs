use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct Settings {
    pub camera_sensitivity: f32,
    pub chromatic_aberation: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            camera_sensitivity: 1.0,
            chromatic_aberation: 1.2,
        }
    }
}
