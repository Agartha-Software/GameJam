mod debug;
mod marker;
mod monster;
mod node;
mod particle;
mod player;
mod settings;
mod speaker;
mod ui;
mod world;

use avian3d::{PhysicsPlugins, prelude::Gravity};
use bevy::prelude::*;
use bevy_aspect_ratio_mask::{AspectRatioMask, AspectRatioPlugin, Resolution};
use bevy_atmosphere::plugin::AtmospherePlugin;
use particle::ParticlePlugin;

use crate::{
    debug::DebugPlugin, marker::MarkerPlugin, monster::MonsterPlugin, player::PlayerPlugin,
    settings::Settings, speaker::SpeakerPlugin, ui::UiPlugin, world::WorldPlugin,
};

fn main() {
    App::new()
        .init_resource::<Settings>()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "550fathom".into(),
                        name: Some("550fathom".into()),
                        ..default()
                    }),
                    ..default()
                }),
            AspectRatioPlugin {
                resolution: Resolution {
                    width: 1600.0,
                    height: 900.0,
                },
                mask: AspectRatioMask {
                    color: Color::BLACK,
                },
            },
            AtmospherePlugin,
            ParticlePlugin,
            PhysicsPlugins::default(),
            DebugPlugin,
            UiPlugin,
            WorldPlugin,
            PlayerPlugin,
            MonsterPlugin,
            MarkerPlugin,
            SpeakerPlugin,
        ))
        .insert_resource(Gravity(Vec3::NEG_Z * 0.3))
        .run();
}
