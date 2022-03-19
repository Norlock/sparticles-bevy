use crate::animations::animation::Animate;
use crate::animations::animation_handler::AnimationOptions;
use crate::animations::animation_handler::StartAnimationAt;
use crate::animations::color_animation::DuoColorAnimation;
use crate::animations::size_animation::SizeAnimation;
use crate::animations::stray_animation::StrayAnimation;
use crate::emitters::diffusion_animation::DiffusionAnimation;
use crate::emitters::emit_color_animation::EmitColorAnimation;
use crate::emitters::emit_speed_animation::EmitSpeedAnimation;
use crate::emitters::emitter::EmitterOptions;
use crate::emitters::emitter::EmitterSize;
use crate::emitters::emitter::Velocity;
use crate::emitters::emitter_animation::EmitterAnimate;
use crate::emitters::emitter_animation_handler::EmitterAnimationHandler;
use crate::emitters::loose_movement_animation::LooseMovementAnimation;
use crate::emitters::randomize_size_animation::RandomizeSizeAnimation;
use crate::emitters::sway_animation::SwayAnimation;
use crate::forces::accelerating_force::AcceleratingForce;
use crate::forces::constant_force::ConstantForce;
use crate::forces::force_handler::ForceHandler;
use crate::forces::gravitational_force::GravitationalForce;
use crate::math::velocity;
use crate::trails::trail_animation::TrailAnimation;
use crate::trails::trail_animation::TrailOptions;
use crate::trails::trail_handler::TrailHandler;
use bevy::math::Vec3;
use bevy::render::color::Color;
use std::time::Duration;

pub fn shimmer_animations() -> AnimationOptions {
    let mut animations: Vec<Box<dyn Animate + Sync + Send>> = Vec::new();

    animations.push(Box::new(DuoColorAnimation {
        color_from: Color::rgba(0.5, 1., 0.5, 1.),
        color_to: Color::rgba(0., 0., 1., 1.),
        from_ms: 1000,
        until_ms: 3000,
    }));

    animations.push(Box::new(DuoColorAnimation {
        color_from: Color::rgba(0., 0., 1., 1.),
        color_to: Color::rgba(0., 0., 1., 0.),
        from_ms: 3000,
        until_ms: 4000,
    }));

    animations.push(Box::new(DuoColorAnimation {
        color_from: Color::rgba(0., 0., 1., 0.),
        color_to: Color::rgba(0., 0., 1., 1.),
        from_ms: 4000,
        until_ms: 5000,
    }));

    animations.push(Box::new(StrayAnimation::new(0, 5000, 7.)));

    //animations.push(Box::new(SizeAnimation {
    //from_ms: 2000,
    //until_ms: 3000,
    //start_scale: 1.,
    //end_scale: 1.5,
    //}));

    //animations.push(Box::new(SizeAnimation {
    //from_ms: 3000,
    //until_ms: 4000,
    //start_scale: 1.5,
    //end_scale: 1.,
    //}));

    AnimationOptions::new(5000, StartAnimationAt::RangeMs(0, 1000), animations)
}

pub fn trail_animation() -> TrailHandler {
    let trail_animations = vec![TrailAnimation::new(TrailOptions {
        update_ms: 16,
        opacity_loss_per_update: 1. / 3.,
        diameter_fraction: 0.5,
        from_ms: 0_000,
        until_ms: 10_000,
    })];

    TrailHandler {
        duration_ms: 10_000,
        trail_animations,
    }
}

//pub fn smoke() -> EmitterOptions {
//let mut animations: Vec<Box<dyn Animate>> = Vec::new();

//animations.push(Box::new(MonoColorAnimation {
//color: Color::from_rgba(145, 42, 245, 255),
//from_ms: 0,
//until_ms: 1000,
//}));

//animations.push(Box::new(DuoColorAnimation {
//color_to: Color::from_rgba(145, 42, 245, 255),
//color_from: Color::from_rgba(200, 100, 1, 255),
//from_ms: 1000,
//until_ms: 2000,
//}));

//animations.push(Box::new(SizeAnimation {
//from_ms: 0,
//until_ms: 1000,
//start_radius: 1.,
//end_radius: 3.,
//}));

//let trail_anim_1 = TrailAnimation::new(TrailOptions {
//update_ms: 32,
//opacity_loss_per_update: 1. / 6.,
//diameter_fraction: 0.7,
//from_ms: 0_000,
//until_ms: 3_000,
//});

//let trail_handler = TrailHandler {
//duration_ms: 4000,
//trail_animations: vec![trail_anim_1],
//};

//let animation_options = AnimationOptions::new(4000, StartAnimationAt::Zero, animations);

//let mut force_handler = ForceHandler::new(Duration::from_secs(4));

//force_handler.add(Box::new(ConstantForce {
//from_ms: 0,
//until_ms: 4000,
//nx: 0.021,
//ny: 0.02,
//max_vx: 2.,
//max_vy: 2.,
//}));

//force_handler.add(Box::new(ConstantForce {
//from_ms: 2000,
//until_ms: 2700,
//nx: 0.,
//ny: -0.03,
//max_vx: 0.,
//max_vy: -2.,
//}));

//EmitterOptions {
//emitter_position: Position::new(300., 300.),
//emitter_diameter: 100.,
//emitter_duration: Duration::from_secs(10),
//angle_degrees: 135.,
//emission_distortion_px: 0.,
//delay_between_emission: Duration::from_millis(2500),
//diffusion_degrees: 360.,
//particle_color: Color::from_rgba(200, 100, 1, 255),
//particle_texture: None,
//particles_per_emission: 200,
//particle_lifetime: Duration::from_secs(3),
//particle_radius: 5.,
//particle_mass: 1.,
//particle_speed: 2.2,
//particle_friction_coefficient: 0.01,
//respect_grid_bounds: true,
//particle_animation_options: Some(animation_options),
//force_handler: Some(force_handler),
//emitter_animation_handler: None,
//trail_handler: Some(trail_handler),
////trail_handler: None,
//}
//}

pub fn emitter_animations() -> Option<EmitterAnimationHandler> {
    let loop_ms = 6000;
    //let sway_1 = Box::new(SwayAnimation {
    //from_ms: 0,
    //until_ms: loop_ms,
    //start_elevation_radians: 135_f32.to_radians(),
    //end_elevation_radians: 360_f32.to_radians(),
    //start_bearing_radians: 10_f32.to_radians(),
    //end_bearing_radians: 60_f32.to_radians(),
    //});

    //let sway_2 = Box::new(SwayAnimation {
    //from_ms: 1000,
    //until_ms: 2000,
    //start_elevation_radians: 0.,
    //end_elevation_radians: 135.,
    //});

    //let sway_3 = Box::new(SwayAnimation {
    //from_ms: 2000,
    //until_ms: 3000,
    //start_elevation_radians: 135.,
    //end_elevation_radians: 360.,
    //});

    //let sway_4 = Box::new(SwayAnimation {
    //from_ms: 3000,
    //until_ms: 4000,
    //start_elevation_radians: 0.,
    //end_elevation_radians: 135.,
    //});

    let diffusion_1 = Box::new(DiffusionAnimation {
        from_ms: 0,
        until_ms: loop_ms,
        start_elevation_radians: 10_f32.to_radians(),
        end_elevation_radians: 10_f32.to_radians(),
        start_bearing_radians: 10_f32.to_radians(),
        end_bearing_radians: 90_f32.to_radians(),
    });

    //let diffusion_2 = Box::new(DiffusionAnimation {
    //from_ms: 2000,
    //until_ms: 4000,
    //start_diffusion_degrees: 5.,
    //end_diffusion_degrees: 125.,
    //});

    //let movement_1 = Box::new(LooseMovementAnimation {
    //stray_radians: 5_f32.to_radians(),
    //emitter_mass: 1.,
    //gravitational_force: 1.,
    //base: Vec3::ZERO,
    //base_mass: 10000.,
    //x_range: 15.,
    //z_range: 5.,
    //y_range: 10.,
    //});

    //let movement_2 = Box::new(LooseMovementAnimation {
    //from_ms: 3000,
    //until_ms: 4000,
    //vx: -1.2,
    //vy: -0.6,
    //stray_radians: 2_f32.to_radians(),
    //});

    let color_1 = Box::new(EmitColorAnimation {
        from_ms: 1000,
        until_ms: 3000,
        from_color: Color::rgb(0.7, 0.2, 0.0),
        to_color: Color::rgb(0.7, 0.0, 0.7),
    });

    let speed_1 = Box::new(EmitSpeedAnimation {
        from_ms: 0,
        until_ms: 2000,
        from_speed: 30.,
        to_speed: 40.,
    });

    //let speed_2 = Box::new(EmitSpeedAnimation {
    //from_ms: 3000,
    //until_ms: 4000,
    //from_speed: 1.5,
    //to_speed: 4.0,
    //});

    //let randomize_size_1 = Box::new(RandomizeSizeAnimation {
    //min_radius: 0.1,
    //max_radius: 0.7,
    //});

    let animations: Vec<Box<dyn EmitterAnimate + Sync + Send>> =
        vec![diffusion_1, color_1, speed_1];

    Some(EmitterAnimationHandler::new(loop_ms, animations))
}

//pub fn another_emitter() -> EmitterOptions {
//let mut animations: Vec<Box<dyn Animate>> = Vec::new();

//animations.push(Box::new(DuoColorAnimation {
//color_from: Color::from_rgba(0, 10, 20, 255),
//color_to: Color::from_rgba(0, 61, 152, 255),
//from_ms: 1000,
//until_ms: 2000,
//}));

//animations.push(Box::new(DuoColorAnimation {
//color_from: Color::from_rgba(0, 61, 162, 255),
//color_to: Color::from_rgba(102, 0, 102, 255),
//from_ms: 1000,
//until_ms: 3_000,
//}));

//animations.push(Box::new(StrayAnimation::new(1_000, 3_000, 10.)));

//let animation_options = AnimationOptions::new(3_000, StartAnimationAt::Zero, animations);

//let trail_animations = vec![TrailAnimation::new(TrailOptions {
//from_ms: 0,
//until_ms: 3_000,
//update_ms: 16,
//opacity_loss_per_update: 0.15,
//diameter_fraction: 0.7,
//})];

//let trail_handler = Some(TrailHandler {
//duration_ms: 3_000,
//trail_animations,
//});

//let mut force_handler = ForceHandler::new(Duration::from_secs(10));
//force_handler.add(Box::new(GravitationalForce {
//from_ms: 0,
//until_ms: 5000,
//gravitation_force: -0.3,
//dead_zone: 30.,
//mass: 1000.,
//start: Point(400., 400.),
//end: Point(400., 800.),
//}));

//force_handler.add(Box::new(GravitationalForce {
//from_ms: 5000,
//until_ms: 10000,
//gravitation_force: -0.4,
//dead_zone: 20.,
//mass: 1000.,
//start: Point(400., 800.),
//end: Point(400., 400.),
//}));

//EmitterOptions {
//emitter_position: Position::new(300., 200.),
//emitter_diameter: 100.,
//emitter_duration: Duration::from_secs(10),
//angle_degrees: 135.,
//emission_distortion_px: 3.,
//delay_between_emission: Duration::from_millis(10),
//diffusion_degrees: 70.,
//particle_color: Color::from_rgba(10, 0, 250, 255),
//particle_texture: None,
//particles_per_emission: 8,
//particle_lifetime: Duration::from_secs(4),
//particle_radius: 3.,
//particle_mass: 1.,
//particle_friction_coefficient: 0.007,
//particle_speed: 2.5,
//respect_grid_bounds: false,
//particle_animation_options: Some(animation_options),
//force_handler: Some(force_handler),
//emitter_animation_handler: sway_and_diffusion_animation(),
//trail_handler,
//}
//}

pub fn random_forces() -> Option<ForceHandler> {
    let forces_length = Duration::from_secs(6).as_millis();
    let mut force_handler = ForceHandler::new(forces_length);

    //force_handler.add(Box::new(AcceleratingForce {
    //from_ms: 0,
    //until_ms: 1000,
    //nx: 0.022,
    //ny: 0.02,
    //nz: 0.,
    //max_vx: 0.2,
    //max_vy: 0.2,
    //max_vz: 0.,
    //}));

    force_handler.add(Box::new(AcceleratingForce {
        from_ms: 2_000,
        until_ms: 3_000,
        nx: 0.,
        ny: 10.,
        nz: 0.,
        max_vx: 0.0,
        max_vy: 50.,
        max_vz: 0.,
    }));

    //force_handler.add(Box::new(GravitationalForce {
    //from_ms: 0,
    //until_ms: 3000,
    //gravitation_force: -0.01,
    //dead_zone: 5.,
    //mass: 100.,
    //start: Vec3::new(0., 0., 0.),
    //end: Vec3::new(0., 0., 0.),
    //}));

    //force_handler.add(Box::new(GravitationalForce {
    //from_ms: 000,
    //until_ms: forces_length,
    //gravitational_force: 1.5,
    //dead_zone: 5.,
    //mass: 20000.,
    //start: Vec3::new(0., 0., 0.),
    //end: Vec3::new(0., 0., 0.),
    //}));

    Some(force_handler)
}
