use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct Settings {
    pub camera_sensitivity: f32,

    // pub fo
}

// pub stru

impl Default for Settings {
    fn default() -> Self {
        Self {
            camera_sensitivity: 1.0,
        }
    }
}
