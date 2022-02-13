#![allow(dead_code)]

use crate::emitters::emitter::EmitterSize;
use crate::pattern::random_forces;
use crate::pattern::shimmer_animations;
use crate::point::Point;
use bevy_config_cam::ConfigCam;
use bevy_config_cam::MovementSettings;
use bevy_config_cam::PlayerSettings;
use dev_ui::DevUIPlugin;
use emitters::emitter::Bounds;
use std::time::Duration;

//use crate::forces::force::Force;
use crate::position::Position;
use bevy::prelude::*;
use emitters::emitter::{Emitter, EmitterOptions, EmitterPlugin};

mod animations;
mod collision;
//mod container;
mod emitters;
mod instant_extensions;
//mod fill_style;
mod dev_ui;
mod forces;
mod grid;
//mod movement_handler;
mod particle;
mod pattern;
mod point;
mod position;
//mod swarm_emitter;
mod trails;

//use grid::{Grid, GridOptions};

//use fill_style::FillStyle;
//use particle::ParticleAttributes;
//use pattern::{another_emitter, random_forces, shimmer_animations, smoke, trail_animation};
//use position::Position;

fn main() {
    //let position = Position::new(100., 100.);

    //let mut grid = Grid::new(GridOptions {
    //cell_x_count: 10,
    //cell_y_count: 10,
    //possibility_x_count: 10,
    //possibility_y_count: 10,
    //possibility_side_length: 10,
    //position,
    //force_handler: random_forces(),
    //});

    //let attributes = ParticleAttributes {
    //color: Color::from_rgba(0, 255, 255, 255),
    //friction_coefficient: 0.005,
    //diameter: 5.5,
    //elasticity: 1.,
    //mass: 3.8,
    //animation_options: None,
    //};

    //grid.fill(&attributes, 500, FillStyle::WhiteNoise);

    //let texture = load_texture("assets/bubble.png").await.unwrap();
    //let attributes = ParticleAttributes {
    //color: Color::from_rgba(255, 255, 255, 255),
    //texture: Some(texture),
    //friction_coefficient: 0.001,
    //diameter: 6.,
    //elasticity: 1.,
    //mass: 2.0,
    //animation_options: Some(shimmer_animations()),
    //trail_handler: None,
    //};

    //grid.fill(&attributes, 50, FillStyle::WhiteNoise);

    //let attributes = ParticleAttributes {
    ////color: Color::from_rgba(231, 196, 150, 255),
    //color: Color::from_rgba(0, 255, 0, 255),
    //texture: None,
    //friction_coefficient: 0.008,
    //diameter: 7.,
    //elasticity: 1.,
    //mass: 3.,
    ////trail_handler: Some(trail_animation()),
    //trail_handler: None,
    //animation_options: None,
    //};

    //grid.fill(&attributes, 100, FillStyle::WhiteNoise);

    //grid.add_emitter(smoke());
    //grid.add_emitter(another_emitter());

    //let color = Color::from_rgba(0, 26, 51, 255);
    App::new()
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(DevUIPlugin)
        .add_plugin(EmitterPlugin)
        .run();
}

fn setup(mut commands: Commands, meshes: ResMut<Assets<Mesh>>, time: Res<Time>) {
    //let grid = Grid::new(GridOptions {
    //cell_x_count: 10,
    //cell_y_count: 10,
    //cell_z_count: 1,
    //possibility_x_count: 10,
    //possibility_y_count: 10,
    //possibility_z_count: 10,
    //possibility_side_length: 10,
    //position,
    ////force_handler: random_forces(),
    //});

    commands.spawn_bundle(PointLightBundle {
        // transform: Transform::from_xyz(5.0, 8.0, 2.0),
        transform: Transform::from_xyz(1.0, 2.0, 0.0),
        point_light: PointLight {
            intensity: 3000.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::ORANGE,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        //transform: Transform::from_xyz(0., 10., 100.),
        transform: Transform {
            translation: Vec3::new(0., 10., 200.),
            //rotation: Quat::from_rotation_z(90_f32.to_radians()),
            ..Default::default()
        },
        ..Default::default()
    });

    //commands.insert_resource(grid);
    commands.spawn_bundle(UiCameraBundle::default());

    let emitter_position = Position::new(0., 0., 0.);
    let options = EmitterOptions {
        emitter_position,
        emitter_size: EmitterSize {
            length: 8.,
            depth: 8.,
        },
        emitter_duration: Duration::from_secs(10),
        angle_degrees: Point(0., 0.),
        diffusion_degrees: Point(360., 360.),
        emission_distortion: 0.,
        particle_color: Color::Rgba {
            red: 0.5,
            green: 1.0,
            blue: 0.5,
            alpha: 1.0,
        },
        particles_per_emission: 5,
        delay_between_emission_ms: 100,
        particle_lifetime: Duration::from_secs(5),
        particle_radius: 0.3,
        particle_mass: 1.,
        particle_speed: 0.3,
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
        particle_animation_options: Some(shimmer_animations()),
    };

    let total_elapsed_ms = time.time_since_startup().as_millis();
    Emitter::create(options, &mut commands, meshes, total_elapsed_ms);
}
