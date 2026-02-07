use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::prelude::*;

use crate::player::WorldModelCamera;
use crate::settings::Settings;
use crate::ui::OverlayImage;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 0.3,
                    font: default(),
                    ..default()
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                enabled: false,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    min_fps: 30.0,
                    target_fps: 144.0,
                },
                ..Default::default()
            },
        })
        .add_systems(Update, toggle_debug);
    }
}

fn toggle_debug(
    input: Res<ButtonInput<KeyCode>>,
    mut fps_overlay: ResMut<FpsOverlayConfig>,
    mut fog: Single<&mut DistanceFog, With<WorldModelCamera>>,
    mut image_overlay: Single<&mut Visibility, With<OverlayImage>>,
    mut settings: ResMut<Settings>,
) {
    if input.just_pressed(KeyCode::F1) {
        fps_overlay.enabled = !fps_overlay.enabled;
        fps_overlay.frame_time_graph_config.enabled = !fps_overlay.frame_time_graph_config.enabled;
    }
    if input.just_pressed(KeyCode::F2) {
        settings.debug = !settings.debug;
        if settings.debug == true {
            //fog.color.set_alpha(0.0);
            **image_overlay = Visibility::Hidden;
        } else {
            fog.color.set_alpha(1.0);
            **image_overlay = Visibility::Visible;
        }
    }
}
