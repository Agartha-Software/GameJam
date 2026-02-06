use bevy::{
    asset::Assets,
    camera::{
        Camera, Camera3d, ClearColorConfig, PerspectiveProjection, Projection,
        visibility::{RenderLayers, Visibility},
    },
    color::{Color, palettes::tailwind},
    core_pipeline::tonemapping::Tonemapping,
    ecs::{
        children,
        component::Component,
        system::{Commands, ResMut},
    },
    light::{
        AmbientLight, EnvironmentMapLight, GlobalAmbientLight, NotShadowCaster, VolumetricFog,
    },
    math::{Vec3, primitives::Cuboid},
    mesh::{Mesh, Mesh3d},
    pbr::{DistanceFog, FogFalloff, MeshMaterial3d, StandardMaterial},
    post_process::bloom::Bloom,
    transform::components::Transform,
    utils::default,
};

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct PlayerCamera;

#[derive(Debug, Component)]
struct WorldModelCamera;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
pub const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
pub const VIEW_MODEL_RENDER_LAYER: usize = 1;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    let camera = commands
        .spawn((
            PlayerCamera,
            Transform::from_xyz(0.0, 0.0, 1.6), //.looking_to(Vec3::X, Vec3::Z),
            Visibility::default(),
            children![
                (
                    WorldModelCamera,
                    Camera3d::default(),
                    Camera {
                        clear_color: ClearColorConfig::Custom(Color::srgb_u8(5, 22, 48)),
                        ..default()
                    },
                    Projection::from(PerspectiveProjection {
                        fov: 90.0_f32.to_radians(),
                        ..default()
                    }),
                    Bloom::OLD_SCHOOL,
                    Tonemapping::TonyMcMapface,
                    DistanceFog {
                        color: Color::srgb_u8(5, 22, 48),
                        falloff: FogFalloff::Exponential { density: 0.6 },
                        ..default()
                    },
                ),
                // Spawn view model camera.
                (
                    Camera3d::default(),
                    Camera {
                        // Bump the order to render on top of the world model.
                        order: 1,
                        ..default()
                    },
                    Bloom::OLD_SCHOOL,
                    Projection::from(PerspectiveProjection {
                        fov: 70.0_f32.to_radians(),
                        ..default()
                    }),
                    VolumetricFog {
                        ambient_color: Color::srgb_u8(5, 22, 48),
                        ambient_intensity: 0.0f32,
                        ..default()
                    },
                    // Only render objects belonging to the view model.
                    RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                ),
                // Spawn the player's right arm.
                (
                    Mesh3d(arm),
                    MeshMaterial3d(arm_material),
                    Transform::from_xyz(0.2, -0.1, -0.25),
                    // Ensure the arm is only rendered by the view model camera.
                    RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                    // The arm is free-floating, so shadows would look weird.
                    NotShadowCaster,
                ),
            ],
        ))
        .id();

    commands
        .spawn((Player, Transform::default(), Visibility::default()))
        .add_child(camera);
}
