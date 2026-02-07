pub mod debug;
pub mod monster;
pub mod particle;
pub mod player;
pub mod settings;
pub mod speaker;
pub mod ui;
pub mod world;

use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_aspect_ratio_mask::{AspectRatioMask, AspectRatioPlugin, Resolution};
use bevy_atmosphere::plugin::AtmospherePlugin;
use particle::ParticlePlugin;

use crate::{
    debug::DebugPlugin, monster::MonsterPlugin, player::PlayerPlugin, settings::Settings,
    speaker::spawn_speaker, ui::UiPlugin, world::WorldPlugin,
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
                    width: 16.0,
                    height: 9.0,
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
        ))
        .add_systems(Startup, spawn_speaker)
        .run();
}
