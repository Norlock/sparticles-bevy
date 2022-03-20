#![allow(dead_code)]

use crate::angles::Angles;
use crate::emitters::emitter::EmitterSize;
use crate::pattern::emitter_animations;
use crate::pattern::random_forces;
use crate::pattern::shimmer_animations;
use dev::dev_camera::DevCameraPlugin;
use dev::dev_ui::DevUIPlugin;
use emitters::emitter::Velocity;
use shaders::shader::ShaderPlugin;
use std::time::Duration;

use bevy::prelude::*;
use emitters::emitter::{Emitter, EmitterOptions, EmitterPlugin};

mod angles;
mod animations;
mod collision;
mod dev;
mod emitters;
mod forces;
mod grid;
mod math;
mod particle;
mod pattern;
mod shaders;
mod trails;

fn main() {
    App::new()
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(DevUIPlugin)
        .add_plugin(DevCameraPlugin)
        .add_plugin(ShaderPlugin)
        .add_plugin(EmitterPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(1.0, 2.0, 0.0),
        point_light: PointLight {
            intensity: 3000.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::ORANGE,
            shadows_enabled: false,
            ..Default::default()
        },
        ..Default::default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    commands.spawn_bundle(UiCameraBundle::default());

    let options = EmitterOptions {
        emitter_transform: Transform::from_xyz(0., -20., 0.),
        emitter_size: EmitterSize {
            length: 8.,
            depth: 4.,
        },
        emitter_duration: Duration::from_secs(30),
        angle_degrees: Angles::new(45., 0.),
        diffusion_degrees: Angles::new(45., 45.),
        emission_distortion: 0.,
        //emitter_velocity: Velocity::new(10., -15., 10.),
        emitter_velocity: Velocity::zero(),
        particle_color: Color::Rgba {
            red: 0.5,
            green: 1.0,
            blue: 0.5,
            alpha: 1.0,
        },
        particles_per_emission: 200,
        delay_between_emission_ms: 100,
        particle_lifetime: Duration::from_secs(5),
        particle_radius: 0.1,
        particle_mass: 1.,
        particle_speed: 20.,
        particle_friction_coefficient: 0.005,
        force_handler: random_forces(),
        bounds: None,
        //bounds: Some(Bounds {
        //start_x: None,
        //start_y: Some(0.),
        //start_z: None,
        //end_x: None,
        //end_y: None,
        //end_z: None,
        //}),
        emitter_animation_handler: emitter_animations(),
        particle_animation_options: Some(shimmer_animations()),
    };

    let total_elapsed_ms = time.time_since_startup().as_millis();
    Emitter::create(options, &mut commands, meshes, materials, total_elapsed_ms);
}
