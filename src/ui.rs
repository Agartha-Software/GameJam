use bevy::prelude::*;
use bevy_aspect_ratio_mask::Hud;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);
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
