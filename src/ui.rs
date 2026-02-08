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
        parent.spawn((
            Node {
                top: Val::Px((900.0 - 70.0) / 2.),
                left: Val::Px((1600.0 - 70.0) / 2.),
                position_type: PositionType::Absolute,
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
        parent.spawn((
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                left: Val::Vw(30.0),
                width: Val::Vw(20.0),
                height: Val::Vh(7.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            InvalidText,
            Visibility::Hidden,
            Text2d::new("The company isn't proud yet, unmarked oil sources remains..."),
        ));
        parent.spawn((
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                left: Val::Vw(30.0),
                width: Val::Vw(20.0),
                height: Val::Vh(7.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            Visibility::Hidden,
            ValidText,
            Text2d::new("You did a great job! You might even get a raise."),
        ));
    });
}

#[derive(Component)]
struct ValidText;

#[derive(Component)]
struct InvalidText;

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
