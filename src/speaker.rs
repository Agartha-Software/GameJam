use avian3d::prelude::LinearVelocity;
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
    transform::components::Transform,
};

pub struct SpeakerPlugin;

impl Plugin for SpeakerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, load_speaker_gltf)
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

#[derive(Resource)]
struct SpeakerAssets {
    model: Handle<Gltf>,
    material_blink: Option<Handle<StandardMaterial>>,
}

fn load_speaker_gltf(mut commands: Commands, assets: Res<AssetServer>) {
    let model = assets.load::<Gltf>("speaker.glb");

    commands.insert_resource(SpeakerAssets {
        model,
        material_blink: None,
    });
}

fn spawn_speaker(
    mut commands: Commands,
    gltf: Res<Assets<Gltf>>,
    mut speaker_assets: ResMut<SpeakerAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut stop: Local<bool>,
) {
    if *stop {
        return;
    }

    let Some(gltf) = gltf.get(&speaker_assets.model) else {
        return;
    };

    let Some(blink) = gltf.named_materials.get("light") else {
        return;
    };

    let Some(material) = materials.get_mut(blink) else {
        return;
    };

    material.emissive = LinearRgba::rgb(0.0, 4.0, 0.0);

    speaker_assets.material_blink = Some(blink.clone());

    *stop = true;

    commands.spawn((
        SceneRoot(gltf.scenes[0].clone()),
        Speaker::default(),
        avian3d::dynamics::prelude::RigidBody::Kinematic,
        LinearVelocity::default(),
        Transform::from_xyz(-64.0, -81.0, 22.25),
    ));
}
