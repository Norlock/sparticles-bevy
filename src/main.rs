#![allow(dead_code)]

use std::time::Duration;

use crate::position::Position;
use bevy::prelude::*;
use emitters::emitter::{Emitter, EmitterOptions, EmitterPlugin};
//mod animations;
mod collision;
//mod container;
mod emitters;
//mod fill_style;
mod force;
mod grid;
//mod movement_handler;
mod particle;
//mod pattern;
mod point;
mod position;
//mod swarm_emitter;
//mod trails;

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
        .add_plugin(EmitterPlugin)
        .run();
}

fn setup(mut commands: Commands) {
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

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(50., 0., 200.),
        ..Default::default()
    });

    //commands.insert_resource(grid);
    commands.spawn_bundle(UiCameraBundle::default());

    let emitter_position = Position::new(10., 0., 0.);
    let options = EmitterOptions {
        emitter_position,
        emitter_diameter: 120.,
        emitter_duration: Duration::from_secs(10),
        angle_degrees: 10.,
        diffusion_degrees: 60.,
        emission_distortion: 0.,
        particle_color: Color::Rgba {
            red: 0.5,
            green: 1.0,
            blue: 0.5,
            alpha: 1.,
        },
        particles_per_emission: 10,
        delay_between_emission: Duration::from_millis(10),
        particle_lifetime: Duration::from_secs(3),
        particle_radius: 5.,
        particle_mass: 1.,
        particle_speed: 0.1,
        particle_friction_coefficient: 0.001,
        bounds: None,
    };

    let emitter = Emitter::new(options);
    commands.spawn().insert(emitter);
}
