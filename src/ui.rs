use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_aspect_ratio_mask::Hud;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ui, init_mouse))
            .add_systems(Update, grab_mouse);
    }
}

fn init_mouse(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false;
}

fn grab_mouse(mut cursor_options: Single<&mut CursorOptions>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::Escape) {
        if cursor_options.visible {
            cursor_options.grab_mode = CursorGrabMode::Locked;
        } else {
            cursor_options.grab_mode = CursorGrabMode::None;
        }
        cursor_options.visible = !cursor_options.visible;
    }
}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>, hud: Res<Hud>) {
    commands.entity(hud.0).with_children(|parent| {
        parent.spawn((
            Node {
                display: Display::Flex,
                align_self: AlignSelf::Stretch,
                justify_self: JustifySelf::Stretch,
                ..Default::default()
            },
            ImageNode::new(asset_server.load("overlay.png")),
        ));
    });
}
