use std::{f32::consts::PI, ops::Neg};

use avian3d::prelude::{LayerMask, LinearVelocity, RayCaster, RayHits, SpatialQueryFilter};
use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    camera::visibility::Visibility,
    color::{Color, LinearRgba},
    ecs::{
        component::Component,
        entity::Entity,
        query::Without,
        resource::Resource,
        system::{Commands, Local, Query, Res, ResMut},
    },
    gltf::Gltf,
    math::{Dir3, FloatPow, Vec3},
    pbr::StandardMaterial,
    scene::SceneRoot,
    time::Time,
    transform::components::Transform,
};

use crate::{player::PLAYER_FLOOR_LAYER, speaker::Speaker};

const MONSTER_GRAVITY: f32 = 5.0;

const MONSTER_MAX_STALKING_SPEED: f32 = 40.0 / 3.6;
const MONSTER_MIN_STALKING_SPEED: f32 = 20.0 / 3.6;

const MONSTER_TURN_AMOUNT: f32 = 0.1;

const MONSTER_HOVER_HEIGHT: f32 = 8.0;
const MONSTER_RAY_PRE_LEN: f32 = 4.0;

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

#[derive(Component)]
pub struct Monster {
    agro: MonsterAgro,
    agressivity: f32,
    direction: f32,
    caster: Entity,
}

#[derive(Resource)]
pub struct MonsterAssets {
    pub model: Handle<Gltf>,
    pub material_eyes: Handle<StandardMaterial>,
    pub material_teeth: Handle<StandardMaterial>,
}

pub fn monster_system(
    time: Res<Time>,
    monsters: Query<(&mut Monster, &mut Transform, &mut LinearVelocity), Without<Speaker>>,
    speakers: Query<(&Speaker, &Transform)>,
    mut casters: Query<(&RayHits, &mut Transform), (Without<Speaker>, Without<Monster>)>,
) {
    let sounds = speakers.iter().map(|(s, t)| (s, &t.translation));
    for (mut monster, mut transform, mut velocity) in monsters {
        if let Ok((rays, mut caster_transform)) = casters.get_mut(monster.caster) {
            monster.behavior(&time, &mut transform, &mut velocity, rays, sounds.clone());
            caster_transform.translation = transform.translation;
        }
    }
}

impl Monster {
    pub fn behavior<'a, I: IntoIterator<Item = (&'a Speaker, &'a Vec3)>>(
        &mut self,
        time: &Time,
        transform: &mut Transform,
        velocity: &mut LinearVelocity,
        rays: &RayHits,
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
                velocity.0 += v * MONSTER_GRAVITY * time.delta_secs() * time.delta_secs();
                let (v, m) = velocity.0.normalize_and_length();
                velocity.0 = v * m.clamp(MONSTER_MIN_STALKING_SPEED, MONSTER_MAX_STALKING_SPEED);

                let mut distance =
                    rays.first().map(|hit| hit.distance).unwrap_or(-10.0) - MONSTER_RAY_PRE_LEN;
                transform.translation.z += distance.neg().max(0.0);
                distance += velocity.z * 1.0;
                velocity.0.z +=
                    (MONSTER_HOVER_HEIGHT - distance).max(0.0).squared() * time.delta_secs();
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

    *materials.get_mut(eyes).unwrap() = materials
        .get(&monster_assets.material_eyes)
        .unwrap()
        .clone();
    *materials.get_mut(teeth).unwrap() = materials
        .get(&monster_assets.material_teeth)
        .unwrap()
        .clone();

    let floor_cast = RayCaster::new(Vec3::Z * MONSTER_RAY_PRE_LEN, Dir3::NEG_Z)
        .with_max_hits(1)
        .with_query_filter(SpatialQueryFilter {
            mask: LayerMask::NONE | PLAYER_FLOOR_LAYER,
            excluded_entities: Default::default(),
        });

    let caster = commands.spawn((Transform::default(), floor_cast)).id();

    commands.spawn((
        SceneRoot(gltf.scenes[0].clone()),
        Monster {
            caster,
            agro: MonsterAgro::Stalking,
            agressivity: 0.0,
            direction: 0.0,
        },
        avian3d::dynamics::prelude::RigidBody::Kinematic,
        LinearVelocity::from(Vec3::new(0.0, 5.0, 1.0)),
        Transform::from_xyz(20.0, 3.0, 30.0),
        Visibility::default(),
    ));
}
