use std::f32::consts::PI;

use avian3d::prelude::LinearVelocity;
use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    camera::visibility::Visibility,
    color::{Color, LinearRgba},
    ecs::{
        component::Component,
        query::Without,
        resource::Resource,
        system::{Commands, Local, Query, Res, ResMut},
    },
    gltf::Gltf,
    math::Vec3,
    pbr::StandardMaterial,
    scene::SceneRoot,
    time::Time,
    transform::components::Transform,
};

use crate::speaker::Speaker;

const MONSTER_GRAVITY: f32 = 5.0;

const MONSTER_MAX_STALKING_SPEED: f32 = 40.0 / 3.6;

const MONSTER_TURN_AMOUNT: f32 = 0.1;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, load_monster_gltf);
        // app.add_systems(Startup, spawn_monster);
        app.add_systems(Update, spawn_monster);
        app.add_systems(Update, monster_system);
    }
}

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

#[derive(Resource)]
pub struct MonsterAssets {
    pub model: Handle<Gltf>,
    // pub material_base: Handle<StandardMaterial>,
    pub material_eyes: Handle<StandardMaterial>,
    pub material_teeth: Handle<StandardMaterial>,
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
            _ => {} // MonsterAgro::Hunting => todo!(),
                    // MonsterAgro::Bored => todo!(),
        }

        velocity.0 = velocity.0.rotate_axis(
            Vec3::Z,
            self.direction * MONSTER_TURN_AMOUNT * time.delta_secs(),
        );
        transform.rotation = Transform::default()
            .looking_to(-velocity.0, Vec3::Z)
            .rotation;
    }
}

pub fn load_monster_gltf(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let model = assets.load::<Gltf>("anglershark.glb");

    let mut material_eyes = StandardMaterial::default();
    material_eyes.base_color = Color::linear_rgb(50.0, 50.0, 50.0);
    material_eyes.emissive = LinearRgba::rgb(100.0, 100.0, 100.0);
    let material_eyes = materials.add(material_eyes);

    let mut material_teeth = StandardMaterial::default();
    material_teeth.base_color = Color::linear_rgb(10.0, 10.0, 10.0);
    material_teeth.emissive = LinearRgba::rgb(10.0, 10.0, 10.0);
    let material_teeth = materials.add(material_teeth);

    commands.insert_resource(MonsterAssets {
        model,
        material_eyes,
        material_teeth,
    });
}

pub fn spawn_monster(
    mut commands: Commands,
    monster_assets: Res<MonsterAssets>,
    gltf: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut loaded: Local<bool>,
) {
    if *loaded {
        return;
    }

    let Some(gltf) = gltf.get(&monster_assets.model) else {
        return;
    };
    *loaded = true;

    let Some(eyes) = gltf.named_materials.get("eyes") else {
        return;
    };
    let Some(teeth) = gltf.named_materials.get("teeth") else {
        return;
    };

    let Some(base) = gltf.named_materials.get("base") else {
        return;
    };

    *materials.get_mut(eyes).unwrap() = materials
        .get(&monster_assets.material_eyes)
        .unwrap()
        .clone();
    *materials.get_mut(teeth).unwrap() = materials
        .get(&monster_assets.material_teeth)
        .unwrap()
        .clone();

    commands.spawn((
        SceneRoot(gltf.scenes[0].clone()),
        Monster::default(),
        avian3d::dynamics::prelude::RigidBody::Kinematic,
        LinearVelocity::from(Vec3::new(0.0, 5.0, 1.0)),
        Transform::from_xyz(20.0, 3.0, 5.0),
        Visibility::default(),
    ));
}
