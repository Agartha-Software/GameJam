use std::f32::consts::PI;

use avian3d::prelude::LinearVelocity;
use bevy::{
    asset::{AssetServer, Assets}, color::Color, ecs::{
        component::Component,
        query::Without,
        system::{Commands, Query, Res, ResMut},
    }, gltf::GltfAssetLabel, math::Vec3, mesh::Mesh, pbr::StandardMaterial, scene::{Scene, SceneRoot}, time::Time, transform::components::Transform
};

use crate::speaker::Speaker;

const MONSTER_GRAVITY: f32 = 5.0;

const MONSTER_MAX_STALKING_SPEED: f32 = 40.0 / 3.6;

const MONSTER_TURN_AMOUNT: f32 = 0.1;

#[derive(Default)]
pub enum MonsterAgro {
    #[default]
    Stalking,
    Hunting,
    Bored,
}

#[derive(Component, Default)]
pub struct Monster {
    agro: MonsterAgro,
    agressivity: f32,
    direction: f32,
}

pub fn monster_system(
    time: Res<Time>,
    monsters: Query<(&mut Monster, &mut Transform, &mut LinearVelocity), Without<Speaker>>,
    speakers: Query<(&Speaker, &Transform)>,
) {
    let sounds = speakers.iter().map(|(s, t)| (s, &t.translation));
    for (mut monster, mut transform, mut velocity) in monsters {
        monster.behavior(&time, &mut transform, &mut velocity, sounds.clone());
    }
}

impl Monster {
    pub fn behavior<'a, I: IntoIterator<Item = (&'a Speaker, &'a Vec3)>>(
        &mut self,
        time: &Time,
        transform: &mut Transform,
        velocity: &mut LinearVelocity,
        sounds: I,
    ) {
        let sounds = sounds.into_iter();
        let Some(speaker) = sounds
            .map(|(s, v)| {
                let v = v - (transform.translation - Vec3::new(0.0, 0.0, 0.5));
                (s.loudness(&v), s, v)
            })
            .reduce(|a, b| a.0.lt(&b.0).then_some(b).unwrap_or(a))
        else {
            self.agro = MonsterAgro::Bored;
            return;
        };

        let random_direction = rand::random_range::<f32, _>(-1.0..=1.0);

        self.direction += random_direction;
        self.direction = self.direction.clamp(-PI * 2.0, PI * 2.0);

        match self.agro {
            MonsterAgro::Stalking => {
                let (loudness, speaker, v) = speaker;
                if v.z > 0.0 {
                    velocity.0.z *= -1.0;
                }
                velocity.0 += v * MONSTER_GRAVITY * time.delta_secs() * time.delta_secs();
                let (v, m) = velocity.0.normalize_and_length();
                velocity.0 = v * m.min(MONSTER_MAX_STALKING_SPEED);
            }
            MonsterAgro::Hunting => todo!(),
            MonsterAgro::Bored => todo!(),
        }

        velocity.0 = velocity
            .0
            .rotate_axis(Vec3::Z, self.direction * MONSTER_TURN_AMOUNT * time.delta_secs());
        transform.rotation = Transform::default().looking_to(-velocity.0, Vec3::Z).rotation;
    }
}

pub fn spawn_monster(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let shark = assets.load::<Scene>(GltfAssetLabel::Scene(0).from_asset("anglershark.glb"));

    // let shark_tx = assets.load::<Image>("anglershark.png");

    // let mut base_mat = StandardMaterial::default();

    // base_mat.base_color = Color::linear_rgb(0.9, 0.9, 0.9);

    // base_mat.base_color_texture = Some(shark_tx);

    // let base_mat = materials.add(base_mat);

    commands.spawn((
        SceneRoot(shark),
        Monster::default(),
        avian3d::dynamics::prelude::RigidBody::Kinematic,
        LinearVelocity::from(Vec3::new(0.0, 5.0, 1.0)),
        Transform::from_xyz(20.0, 3.0, 5.0),
        Visibility::default(),
    ));
}
