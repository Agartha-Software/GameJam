use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::player::{Player, spawn_player};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(Startup, setup.after(spawn_player));
    }
}

fn setup(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    player: Single<Entity, With<Player>>,
) {
    // Define a color gradient from red to transparent black
    let mut gradient_color = bevy_hanabi::Gradient::new();
    gradient_color.add_key(0.0, Vec4::new(0.2, 0.2, 0.2, 0.));
    gradient_color.add_key(0.5, Vec4::new(0.2, 0.2, 0.2, 0.7));
    gradient_color.add_key(1.0, Vec4::new(0.2, 0.2, 0.2, 0.));

    let mut gradient_size = bevy_hanabi::Gradient::new();
    gradient_size.add_key(0.0, Vec3::new(0.02, 0.02, 0.02));
    gradient_size.add_key(0.5, Vec3::new(0.05, 0.05, 0.05));
    gradient_size.add_key(1.0, Vec3::new(0.02, 0.02, 0.02));

    // Create a new expression writer
    let writer = ExprWriter::new();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::new(0.0, 0.0, 1.0)).expr(),
        radius: writer.lit(15.).expr(),
        dimension: ShapeDimension::Volume,
    };

    // The velocity is random in any direction
    let vel = writer.rand(VectorType::VEC3F);
    let vel = vel * writer.lit(2.) - writer.lit(1.); // remap [0:1] to [-1:1]
    let vel = vel.normalized();
    let speed = writer.lit(0.2); //.uniform(writer.lit(4.));
    let vel = (vel * speed).expr();
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, vel);

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = writer.lit(10.).expr(); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // // Every frame, add a gravity-like acceleration downward
    // let accel = writer.lit(Vec3::new(0., 0., -3.)).expr();
    // let update_accel = AccelModifier::new(accel);

    let rotation = (writer.rand(ScalarType::Float) * writer.lit(std::f32::consts::TAU)).expr();
    let init_rotation = SetAttributeModifier::new(Attribute::F32_0, rotation);

    let rotation_attr = writer.attr(Attribute::F32_0).expr();

    let module = writer.finish();

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        SpawnerSettings::rate(15.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("dust")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .init(init_rotation)
    .render(OrientModifier {
        mode: OrientMode::FaceCameraPosition,
        rotation: Some(rotation_attr),
    })
    .render(SizeOverLifetimeModifier {
        gradient: gradient_size.into(),
        screen_space_size: false,
    })
    // .update(update_accel)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier {
        gradient: gradient_color.into(),
        ..default()
    });

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    let particle = commands.spawn(ParticleEffect::new(effect_handle)).id();

    commands.entity(*player).add_child(particle);
}
