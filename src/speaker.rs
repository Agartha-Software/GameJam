use avian3d::prelude::LinearVelocity;
use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    math::{Vec3, primitives},
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform,
};

#[derive(Component)]
pub struct Speaker {
    pub volume: f32,
    pub active: bool,
}

impl Default for Speaker {
    fn default() -> Self {
        Self {
            volume: 4.0,
            active: true,
        }
    }
}

impl Speaker {
    pub fn loudness(&self, distance: &Vec3) -> f32 {
        if !self.active {
            return 0.0;
        }

        self.volume / (10.0f32.powf(distance.length()))
    }
}

pub fn spawn_speaker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(primitives::Cuboid::new(0.5, 0.5, 0.5));

    let mat = materials.add(StandardMaterial::from_color(Color::srgb(0.7, 0.4, 0.12)));

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(mat),
        Speaker::default(),
        avian3d::dynamics::prelude::RigidBody::Kinematic,
        LinearVelocity::default(),
        Transform::from_xyz(3.0, 1.0, 0.5),
    ));
}
