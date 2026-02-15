use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_aspect_ratio_mask::Hud;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ui, init_mouse))
            .add_systems(Update, (grab_mouse, move_overlay));
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

#[derive(Component, Default)]
pub struct OverlayImage {
    accu: Vec2,
}

#[derive(Component)]
pub struct Cursor;

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>, hud: Res<Hud>) {
    commands.entity(hud.0).with_children(|parent| {
        parent.spawn((
            Node {
                align_self: AlignSelf::Stretch,
                justify_self: JustifySelf::Stretch,
                ..Default::default()
            },
            Visibility::Visible,
            ImageNode::new(asset_server.load("overlay.png")),
            Outline::new(Val::Px(200.0), Val::Px(0.0), Color::BLACK),
            OverlayImage::default(),
        ));
        parent
            .spawn((Node {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                ..default()
            },))
            .with_children(|centered| {
                centered.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        display: Display::None,
                        ..default()
                    },
                    Text::new("-"),
                    TextFont::from_font_size(12.0),
                    TextLayout::new_with_justify(Justify::Center).with_no_wrap(),
                    TextColor(Color::WHITE),
                    CenteredText,
                ));

                centered.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        width: Val::Px(70.0),
                        height: Val::Px(70.0),
                        ..Default::default()
                    },
                    Visibility::Hidden,
                    Cursor,
                    ImageNode {
                        image: asset_server.load("hand_icon.png"),
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ));
            });
    });
}

#[derive(Component)]
pub struct CenteredText;

fn move_overlay(
    overlay: Single<(&mut Node, &mut OverlayImage), With<OverlayImage>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    let (mut node, mut overlay_data) = overlay.into_inner();
    overlay_data.accu += accumulated_mouse_motion.delta / 4.0;
    overlay_data.accu = overlay_data
        .accu
        .clamp(Vec2::new(-80.0, -30.0), Vec2::new(80.0, 30.0));
    let accu = overlay_data.accu;
    overlay_data.accu /= 1. + 2. * 0.3 * accu.abs() * time.delta_secs();

    node.left = Val::Px(-overlay_data.accu.x);
    node.top = Val::Px(-overlay_data.accu.y + ((time.elapsed_secs() * 1.2).sin() * 4.0));
}
