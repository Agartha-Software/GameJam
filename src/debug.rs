use avian3d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::prelude::*;
use bevy::ui_render::UiDebugOptions;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::node::OilNode;
use crate::player::WorldModelCamera;
use crate::player::marker::PLAYER_PLACE_DIST;
use crate::settings::Settings;
use crate::ui::OverlayImage;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 30.,
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
        .add_plugins((EguiPlugin::default(), PhysicsDebugPlugin))
        .add_plugins(WorldInspectorPlugin::new().run_if(if_debug_display))
        .add_systems(
            Update,
            (
                toggle_debug,
                display_oil_nodes_debug.run_if(if_debug_display),
            ),
        );
    }
}

fn if_debug_display(settings: Res<Settings>) -> bool {
    settings.debug_display
}

fn display_oil_nodes_debug(mut gizmos: Gizmos, oil_nodes: Query<&GlobalTransform, With<OilNode>>) {
    for oil_node in oil_nodes {
        gizmos.sphere(
            oil_node.translation(),
            PLAYER_PLACE_DIST,
            Color::srgb(1., 0.7, 0.7),
        );
    }
}

fn toggle_debug(
    input: Res<ButtonInput<KeyCode>>,
    mut fps_overlay: ResMut<FpsOverlayConfig>,
    mut cam: Single<(&mut DistanceFog, &mut Projection), With<WorldModelCamera>>,
    mut image_overlay: Single<&mut Visibility, With<OverlayImage>>,
    mut settings: ResMut<Settings>,
    mut store: ResMut<GizmoConfigStore>,
    mut debug_ui: ResMut<UiDebugOptions>,
) {
    if input.just_pressed(KeyCode::F1) {
        fps_overlay.enabled = !fps_overlay.enabled;
        fps_overlay.frame_time_graph_config.enabled = !fps_overlay.frame_time_graph_config.enabled;
    }
    if input.just_pressed(KeyCode::F2) {
        if settings.debug_display {
            settings.debug_display = false;
        } else {
            settings.debug = !settings.debug;
        }
    }
    if input.just_pressed(KeyCode::F3) {
        settings.debug_display = !settings.debug_display;
        settings.debug = settings.debug_display;
    }
    if settings.debug == true {
        **image_overlay = Visibility::Hidden;
    } else {
        **image_overlay = Visibility::Visible;
    }
    debug_ui.enabled = settings.debug && !settings.debug_display;

    let phys_config = store.config_mut::<PhysicsGizmos>().0;
    if settings.debug_display == true {
        cam.0.color.set_alpha(0.0);
        phys_config.enabled = true;
    } else {
        cam.0.color.set_alpha(1.0);
        phys_config.enabled = false;
    }
}
