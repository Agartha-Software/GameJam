use std::{f32::consts::PI, ops::Neg};

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    audio::AudioPlayer,
    camera::visibility::Visibility,
    color::{Color, LinearRgba},
    ecs::{
        component::Component,
        entity::Entity,
        query::Without,
        resource::Resource,
        system::{Commands, Local, Query, Res, ResMut, Single},
        world::Mut,
    },
    gltf::Gltf,
    math::{Dir3, Vec3},
    pbr::StandardMaterial,
    scene::SceneRoot,
    time::Time,
    transform::components::{GlobalTransform, Transform},
};

use crate::world::GROUND_MASK;
use crate::{player::Player, speaker::Speaker};

const MONSTER_MAX_STALKING_SPEED: f32 = 40.0 / 3.6;
const MONSTER_MIN_STALKING_SPEED: f32 = 15.0 / 3.6;

const MONSTER_TURN_AMOUNT: f32 = 0.01;

/// height to compensate for the body of the monster
const MONSTER_BODY_HEIGHT: f32 = 0.8;

/// height for the center to hover at, including the body's width
const MONSTER_HOVER_HEIGHT: f32 = 8.0;
const MONSTER_RAY_PRE_LEN: f32 = 40.0;

const MONSTER_ANTICIPATION: f32 = 2.0;

const MONSTER_KILL_RADIUS: f32 = 20.0;

const MONSTER_ORBIT_RADIUS: f32 = 40.0;
const MONSTER_ORBIT_RADIUS_2: f32 = MONSTER_ORBIT_RADIUS * MONSTER_ORBIT_RADIUS;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, load_monster_gltf);
        // app.add_systems(Startup, spawn_monster);
        app.add_systems(Update, spawn_monster);
        app.add_systems(Update, monster_system);
    }
}

#[derive(Default, Debug)]
pub enum MonsterAgro {
    #[default]
    Orbit,
    Hunt,
    Fade,
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
    monsters: Query<(Entity, &mut Monster, &mut Transform, &mut LinearVelocity, &mut PlaybackSettings), Without<Speaker>>,
    speakers: Query<(&Speaker, &GlobalTransform)>,
    mut casters: Query<
        (&RayHits, &mut Transform),
        (Without<Speaker>, Without<Monster>, Without<Player>),
    >,
    player: Single<
        (Entity, &mut Player, &GlobalTransform, &mut Transform),
        (Without<Speaker>, Without<Monster>),
    >,
) {
    let sounds = speakers.iter().map(|(s, t)| (s, t.translation()));
    let mut player = player.into_inner();
    for (entity, mut monster, mut transform, mut velocity, mut playback) in monsters {
        if let Ok((rays, mut caster_transform)) = casters.get_mut(monster.caster) {
            monster.behavior(
                &time,
                entity,
                &mut transform,
                &mut velocity,
                &mut playback,
                rays,
                &mut player,
                sounds.clone(),
            );
            caster_transform.translation = transform.translation;
        }
    }
}

impl Monster {
    pub fn behavior<'a, I: IntoIterator<Item = (&'a Speaker, Vec3)>>(
        &mut self,
        time: &Time,
        entity: Entity,
        transform: &mut Transform,
        velocity: &mut LinearVelocity,
        playback: &mut PlaybackSettings,
        rays: &RayHits,
        player: &mut (
            Entity,
            Mut<'_, Player>,
            &GlobalTransform,
            Mut<'_, Transform>,
        ),
        sounds: I,
    ) {
        let sounds = sounds.into_iter();
        let Some(speaker) = sounds
            .map(|(s, pos)| {
                let d = pos - (transform.translation);
                (s.loudness(&d), s, d, pos)
            })
            .reduce(|a, b| a.0.lt(&b.0).then_some(b).unwrap_or(a))
        else {
            self.agro = MonsterAgro::Fade;
            return;
        };

        let random_direction = rand::random_range::<f32, _>(-1.0..=1.0);

        self.direction += random_direction;
        self.direction = self.direction.clamp(-PI * 2.0, PI * 2.0);

        let hover_height: f32;

        let next = match self.agro {
            MonsterAgro::Orbit => {
                playback.volume = bevy::audio::Volume::Linear(2.);
                hover_height = MONSTER_HOVER_HEIGHT;

                let (_loudness, _speaker, d, _pos) = speaker;

                let (d_radius, d_distance) =
                    ((d + Vec3::new(0.0, 0.0, 0.5)) * Vec3::new(1., 1., 2.)).normalize_and_length();

                let attraction = ((d_distance / MONSTER_ORBIT_RADIUS) - 1.0).clamp(0.0, 1.0);

                velocity.0 += attraction * d_radius * time.delta_secs().min(1. / 15.);

                let (v, m) = velocity.0.normalize_and_length();
                velocity.0 = v * m.clamp(MONSTER_MIN_STALKING_SPEED, MONSTER_MAX_STALKING_SPEED);
                None
            }
            MonsterAgro::Fade => {
                playback.volume = bevy::audio::Volume::Linear(1.5);
                hover_height = MONSTER_HOVER_HEIGHT;

                let (_loudness, _speaker, d, _pos) = speaker;

                if d.length() > MONSTER_ORBIT_RADIUS {
                    Some(MonsterAgro::Hunt)
                } else {
                    None
                }
            }
            MonsterAgro::Hunt => {
                playback.volume = bevy::audio::Volume::Linear(4.);
                let (_loudness, _speaker, d, pos) = speaker;

                let (real_dir, real_distance) = d.normalize_and_length();

                let pos = if real_distance < MONSTER_ORBIT_RADIUS {
                    hover_height = MONSTER_BODY_HEIGHT;
                    pos
                } else {
                    hover_height = MONSTER_HOVER_HEIGHT / 2.;
                    pos + MONSTER_HOVER_HEIGHT
                };

                // let d_anticipated =
                //     pos - (transform.translation + velocity.0 * MONSTER_ANTICIPATION * 2.);

                // let (d_anticipated_dir, _d_anticipated_distance) =
                //     d_anticipated.normalize_and_length();

                let swim_fwd = 0.5 * real_dir * time.delta_secs().min(1. / 15.);

                // let swim_lat = 4. * d_anticipated_dir * time.delta_secs().min(1. / 15.);

                velocity.0 += swim_fwd.project_onto(velocity.0);
                // velocity.0 += swim_lat.reject_from(velocity.0);

                // Vec3::rotat

                velocity.0 = velocity
                    .0
                    .rotate_towards(real_dir, time.delta_secs().min(1. / 15.) * (1. + velocity.0.angle_between(real_dir) / PI) / 2.0);

                let (v, m) = velocity.0.normalize_and_length();
                velocity.0 = v * m.clamp(MONSTER_MIN_STALKING_SPEED, MONSTER_MAX_STALKING_SPEED);

                // a miss, the target is behind the velocity and the monster 'just' missed it (radius)
                if d.dot(velocity.0) < 0. && real_distance < MONSTER_ORBIT_RADIUS / 2. {
                    Some(MonsterAgro::Fade)
                } else {
                    None
                }
            }
        };

        // hover
        {
            // ground correction
            let mut distance = rays
                .first()
                .map(|hit| hit.distance)
                .unwrap_or(MONSTER_RAY_PRE_LEN)
                - (MONSTER_RAY_PRE_LEN + MONSTER_BODY_HEIGHT);
            transform.translation.z += distance.neg().max(0.0);

            // hover
            distance += velocity.0.z * MONSTER_ANTICIPATION;
            velocity.0.z += (hover_height - distance).max(0.0) * time.delta_secs().min(1. / 15.);
        }

        velocity.0 = velocity.0.rotate_axis(
            Vec3::Z,
            self.direction * MONSTER_TURN_AMOUNT * time.delta_secs().min(1. / 15.),
        );

        transform.rotation = Transform::default()
            .looking_to(-velocity.0, Vec3::Z)
            .rotation;

        if let Some(next) = next {
            eprintln!("Changing from {:?} to {:?}", self.agro, next);
            self.agro = next;
        }
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
    asset_server: Res<AssetServer>,
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
            mask: GROUND_MASK,
            excluded_entities: Default::default(),
        });

    let caster = commands.spawn((Transform::default(), floor_cast)).id();

    let mut collider = Collider::capsule_endpoints(1.8, (0., 0., -3.).into(), (0., 0., 2.).into());

    // collider.set_scale((1., 2., 1.).into(), 6);

    commands.spawn((
        SceneRoot(gltf.scenes[0].clone()),
        Monster {
            caster,
            agro: MonsterAgro::Hunt,
            agressivity: 0.0,
            direction: 0.0,
        },
        collider,
        avian3d::dynamics::prelude::RigidBody::Kinematic,
        LinearVelocity::from(Vec3::new(0.0, 5.0, 1.0)),
        Transform::from_xyz(20.0, 3.0, 40.0),
        Visibility::default(),
        AudioPlayer::new(asset_server.load("whimper.wav")),
        PlaybackSettings::LOOP
            .with_spatial(true)
            .with_volume(bevy::audio::Volume::Linear(2.)),
    ));
}
