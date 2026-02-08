use bevy::prelude::*;

#[derive(Component)]
pub struct OilNode;

#[derive(Resource)]
pub struct OilNodeResource {
    pub nodes_left: usize,
}

pub fn spawn_world_nodes(mut commands: Commands) {
    commands.spawn_batch([
        (OilNode, Transform::from_xyz(-62.0, -81.0, 22.0)),
        (OilNode, Transform::from_xyz(0.0, 0.0, 0.0)),
        (OilNode, Transform::from_xyz(0.0, 0.0, 0.0)),
        (OilNode, Transform::from_xyz(0.0, 0.0, 0.0)),
    ]);
}
