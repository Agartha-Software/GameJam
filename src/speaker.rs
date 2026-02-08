use avian3d::prelude::*;
use bevy::math::FloatPow;
use bevy::prelude::*;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    color::LinearRgba,
    ecs::{
        component::Component,
        resource::Resource,
        system::{Commands, Local, Res, ResMut},
    },
    gltf::Gltf,
    math::Vec3,
    pbr::StandardMaterial,
    scene::SceneRoot,
    transform::components::{GlobalTransform, Transform},
};

use crate::node::OilNode;
use crate::player::marker::Pickup;

pub struct SpeakerPlugin;

impl Plugin for SpeakerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, load_speaker_gltf)
            .add_systems(PreUpdate, speaker_preupdate)
            .add_systems(Update, spawn_speaker);
    }
}

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

#[derive(Default)]
pub enum SpeakerMode {
    #[default]
    None,
    Blink(f32),
    Ready(Entity),
}

#[derive(Resource)]
pub struct SpeakerResource {
    pub model: Handle<Gltf>,
    pub material_blink: Option<Handle<StandardMaterial>>,
    pub mode: SpeakerMode,
    pub time: f32,
}

fn load_speaker_gltf(mut commands: Commands, assets: Res<AssetServer>) {
    let model = assets.load::<Gltf>("speaker.glb");

    commands.insert_resource(SpeakerResource {
        model,
        material_blink: None,
        mode: SpeakerMode::None,
        time: 0.0,
    });
}

fn spawn_speaker(
    mut commands: Commands,
    gltf: Res<Assets<Gltf>>,
    mut speaker_resource: ResMut<SpeakerResource>,
    mut stop: Local<bool>,
) {
    if *stop {
        return;
    }

    let Some(gltf) = gltf.get(&speaker_resource.model) else {
        return;
    };

    let Some(blink) = gltf.named_materials.get("light") else {
        return;
    };

    speaker_resource.material_blink = Some(blink.clone());

    *stop = true;

    commands.spawn((
        SceneRoot(gltf.scenes[0].clone()),
        Pickup,
        Speaker::default(),
        Collider::cuboid(0.9, 0.75, 1.5),
        RigidBody::Dynamic,
        Transform::from_xyz(-64.0, -81.0, 22.5),
    ));
}

const EMIT_RED: LinearRgba = LinearRgba {
    red: 4.0,
    green: 0.03,
    blue: 0.01,
    alpha: 1.0,
};
const EMIT_GREEN: LinearRgba = LinearRgba {
    red: 0.1,
    green: 4.0,
    blue: 0.03,
    alpha: 1.0,
};

const NODE_SPEAKER_ACTIVATION_DIST: f32 = 5.0;
/// squared distance for easy compute
const NODE_SPEAKER_ACTIVATION_DIST_2: f32 =
    NODE_SPEAKER_ACTIVATION_DIST * NODE_SPEAKER_ACTIVATION_DIST;

fn bias(speaker: &GlobalTransform, node: &GlobalTransform) -> f32 {
    let d = node.translation() - speaker.translation();
    let alignment = speaker.up().dot(d.normalize()) + 2.0;
    let dist = d.length().max(NODE_SPEAKER_ACTIVATION_DIST);
    (alignment / dist).max(0.0)
}

pub fn apply_color(speaker_resource: &SpeakerResource, materials: &mut Assets<StandardMaterial>) {
    let color = match speaker_resource.mode {
        SpeakerMode::None => LinearRgba::BLACK,
        SpeakerMode::Blink(blink) => EMIT_RED * blink,
        SpeakerMode::Ready(_) => EMIT_GREEN,
    };

    let need_change = if let Some(mat_handle) = &speaker_resource.material_blink {
        if let Some(mat) = materials.get(mat_handle) {
            mat.emissive != color
        } else {
            false
        }
    } else {
        false
    };

    if need_change {
        if let Some(mat_handle) = &speaker_resource.material_blink {
            if let Some(mat) = materials.get_mut(mat_handle) {
                mat.emissive = color
            }
        }
    }
}

pub fn speaker_preupdate(
    time: Res<Time>,
    nodes: Query<(Entity, &GlobalTransform), With<OilNode>>,
    speaker: Single<&GlobalTransform, With<Speaker>>,
    mut speaker_resource: ResMut<SpeakerResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    speaker_resource.mode = if let Some((e, n, power)) = nodes
        .iter()
        .map(|(e, n)| (e, n, bias(&speaker, n)))
        .reduce(|a, b| if a.2 < b.2 { b } else { a })
    {
        if n.translation().distance_squared(speaker.translation()) < NODE_SPEAKER_ACTIVATION_DIST_2
        {
            SpeakerMode::Ready(e)
        } else {
            speaker_resource.time += (power * 40.0) * time.delta_secs();
            SpeakerMode::Blink(speaker_resource.time.sin().abs().squared())
        }
    } else {
        SpeakerMode::None
    };

    apply_color(&speaker_resource, &mut materials);
}

pub fn ungrab(
    commands: &mut Commands,
    entity: Entity,
    tm: &mut Transform,
    player_tm: &GlobalTransform,
    player: Entity,
) {
    *tm = player_tm.compute_transform();
    tm.translation.z += 0.7;
    tm.translation += player_tm.up() * 0.5;
    // commands.entity(entity).remove::<ChildOf>();
    commands.entity(player).detach_child(entity);
    commands
        .entity(entity)
        .remove::<ColliderDisabled>()
        .remove::<RigidBodyDisabled>()
        .remove::<GravityScale>();
}

pub fn grab(commands: &mut Commands, entity: Entity, tm: &mut Transform, player: Entity) {
    *tm = Transform::from_xyz(0.3, -0.74, -0.7).with_rotation(Quat::from_euler(
        EulerRot::XYZ,
        -90.0f32.to_radians(),
        0.0,
        0.0,
    ));
    commands.entity(player).add_child(entity);
    commands
        .entity(entity)
        .insert(ColliderDisabled)
        .insert(RigidBodyDisabled)
        .insert(GravityScale(0.));
}
