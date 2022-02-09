//use crate::animations::animation::AnimationData;
//use crate::animations::animation_handler::AnimationHandler;
//use crate::animations::animation_handler::AnimationOptions;
use crate::emitters::emitter_animation_handler::EmitterAnimationHandler;
use crate::force::force::ForceData;
use crate::force::force_handler::ForceHandler;
use bevy::core::FixedTimestep;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
//use crate::trails::trail_animation::TrailData;
//use crate::trails::trail_handler::TrailHandler;
use crate::{position, Position};
use bevy::prelude::*;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::emitter_animation::EmitterData;

pub struct EmitterOptions {
    pub emitter_position: Position,
    pub emitter_diameter: f32,
    pub emitter_duration: Duration,
    pub angle_degrees: f32,
    /// Initial spread factor
    pub diffusion_degrees: f32,
    pub emission_distortion: f32,
    pub particle_color: Color,
    //pub particle_texture: Option<Texture2D>,
    pub particles_per_emission: u32,
    pub delay_between_emission: Duration,
    pub particle_lifetime: Duration,
    pub particle_radius: f32,
    pub particle_mass: f32,

    /// Newton force
    pub particle_speed: f32,
    /// number between 0 and 1, e.g. 0.001
    pub particle_friction_coefficient: f32,
    pub bounds: Option<Bounds>,
    //pub particle_animation_options: Option<AnimationOptions>,
    //pub emitter_animation_handler: Option<EmitterAnimationHandler>,
    //pub force_handler: Option<ForceHandler>,
    //pub trail_handler: Option<TrailHandler>,
}

#[derive(Debug, Component)]
pub struct Bounds {
    start_x: f32,
    start_y: f32,
    start_z: f32,
    end_x: f32,
    end_y: f32,
    end_z: f32,
}

#[derive(Debug, Component)]
pub struct Emitter;
//{
//position: Position,
//particle_texture: Option<Texture2D>,
//trail_handler: Option<TrailHandler>,
//particle_animation_options: Option<AnimationOptions>,
//force_handler: Option<ForceHandler>,
//emitter_animation_handler: Option<EmitterAnimationHandler>,
//pub particle_count: u32,
//}

#[derive(Debug, Component)]
struct Cycle {
    lifetime: Instant,
    duration: Duration,
}

#[derive(Debug, Component)]
struct EmitOptions {
    angle_radians: f32,
    angle_emission_radians: f32,
    diffusion_radians: f32,
    particles_per_emission: u32,
    emission_distortion: f32,
    diameter: f32,
}

#[derive(Debug, Component)]
struct ParticleOptions {
    lifetime_ms: u128,
    radius: f32,
    mass: f32,
    speed: f32,
    friction_coefficient: f32,
    color: Color,
}

#[derive(Debug, Component)]
struct Particle;

#[derive(Debug, Component)]
struct Speed {
    vx: f32,
    vy: f32,
    vz: f32,
}

//lifetime: Arc<Instant>,
//trail_handler: Option<TrailHandler>,
//animation_handler: Option<AnimationHandler>,

const EMIT_RADIANS: f32 = 90_f32 * (std::f32::consts::PI / 181.0f32); // 0 deg will be emitting above

pub struct EmitterPlugin;

impl Plugin for EmitterPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_system_set(
            //SystemSet::new()
            //.with_run_criteria(FixedTimestep::step(0.5))
            //.with_system(update_emitter_system),
            //)
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_system(transform_particle_system)
            .add_system(update_emitter_system);
    }
}

fn transform_particle_system(mut query: Query<(&Speed, &mut Transform), With<Particle>>) {
    //println!("{}", query.iter().count());
    for (speed, mut transform) in query.iter_mut() {
        transform.translation.x += speed.vx;
        transform.translation.y += speed.vy;
        transform.translation.z += speed.vz;
    }
}

fn update_emitter_system(
    mut query: Query<(&Cycle, &EmitOptions, &ParticleOptions, &Position), With<Emitter>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (cycle, emit_options, particle, position) in query.iter_mut() {
        let elapsed = cycle.lifetime.elapsed();
        let overdue = elapsed > cycle.duration;

        if overdue {
            return;
        }

        let mut rng = thread_rng();
        //let lifetime = Arc::new(Instant::now());
        for _ in 0..emit_options.particles_per_emission {
            let emitter_position = gen_abs_range(&mut rng, emit_options.diameter);
            let distortion = gen_dyn_range(&mut rng, emit_options.emission_distortion);
            let x = (position.x + distortion) + emitter_position * emit_options.angle_radians.cos();
            let y = (position.y + distortion) + emitter_position * emit_options.angle_radians.sin();
            let z = 0.;

            let diffusion_delta = gen_dyn_range(&mut rng, emit_options.diffusion_radians);
            let angle_radians = emit_options.angle_emission_radians + diffusion_delta;
            let vx = particle.speed * angle_radians.cos();
            let vy = particle.speed * angle_radians.sin();

            //let animation_handler = AnimationHandler::new(&self.particle_animation_options);

            let speed = Speed { vx, vy, vz: 0. };

            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: particle.radius,
                        ..Default::default()
                    })),
                    material: materials.add(particle.color.into()),
                    transform: Transform {
                        translation: Vec3::new(x, y, z),
                        //scale: Vec3::new(0.1, 0.1, 0.1),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(speed)
                .insert(Particle);
        }
    }
}

impl Emitter {
    pub fn create(options: EmitterOptions, commands: &mut Commands) {
        let EmitterOptions {
            emitter_position,
            emitter_diameter,
            emitter_duration,
            angle_degrees,
            diffusion_degrees,
            emission_distortion,
            particle_color,
            particles_per_emission,
            delay_between_emission,
            particle_lifetime,
            particle_radius,
            particle_mass,
            particle_speed,
            particle_friction_coefficient,
            bounds,
            //emitter_animation_handler,
            //force_handler,
        } = options;

        let angle_radians = angle_degrees.to_radians();
        let angle_emission_radians = angle_radians + EMIT_RADIANS;

        let emit_options = EmitOptions {
            particles_per_emission,
            diffusion_radians: diffusion_degrees.to_radians(),
            angle_radians,
            angle_emission_radians,
            emission_distortion,
            diameter: emitter_diameter,
        };

        let emit_time = Cycle {
            duration: emitter_duration,
            lifetime: Instant::now(),
        };

        let spawn_options = ParticleOptions {
            speed: particle_speed,
            color: particle_color,
            friction_coefficient: particle_friction_coefficient,
            radius: particle_radius,
            lifetime_ms: particle_lifetime.as_millis(),
            mass: particle_mass,
        };

        let mut builder = commands.spawn();
        builder
            .insert(emit_options)
            .insert(emit_time)
            .insert(emitter_position)
            .insert(spawn_options)
            .insert(Emitter);

        if let Some(bounds) = bounds {
            builder.insert(bounds);
        }
        //Self {
        //particles_per_emission,
        //particle_color,
        //particle_mass,
        //particle_radius,
        //position: emitter_position,
        //angle_radians,
        //angle_emission_radians,
        //emission_distortion,
        //particle_lifetime_ms: particle_lifetime.as_millis(),
        //emitter_diameter,
        //emitter_duration: duration,
        //lifetime: Instant::now(),
        //current_emission: -1,
        //delay_between_emission_ms: delay_between_emission.as_millis(),
        //bounds,
        //particle_friction_coefficient,
        //particle_speed,
        ////emitter_animation_handler,
        ////force_handler,
        //delete: false,
        //}
    }

    fn animate_emitter(&mut self, elapsed_ms: u128) {
        //if let Some(anim_handler) = &mut self.emitter_animation_handler {
        //let mut data = EmitterData {
        //delay_between_emission_ms: self.delay_between_emission_ms,
        //particle_speed: self.particle_speed,
        //particle_friction_coefficient: self.particle_friction_coefficient,
        //particles_per_emission: self.particles_per_emission,
        //respect_grid_bounds: self.respect_grid_bounds,
        //emitter_diameter: self.emitter_diameter,
        //particle_color: self.particle_color,
        //emission_distortion: self.emission_distortion,
        //angle_radians: self.angle_radians,
        //diffusion_radians: self.diffusion_radians,
        //particle_radius: self.particle_radius,
        //x: self.x,
        //y: self.y,
        //};

        //anim_handler.animate(&mut data, elapsed_ms);

        //self.angle_emission_radians = data.angle_radians + INVERSE_RADIANS;
        //self.diffusion_radians = data.diffusion_radians;
        //self.x = data.x;
        //self.y = data.y;
        //self.particle_color = data.particle_color;
        //self.particle_speed = data.particle_speed;
        //self.particle_radius = data.particle_radius;
        //}
    }

    //pub fn emit(
    //&mut self,
    //commands: &mut Commands,
    //meshes: &mut ResMut<Assets<Mesh>>,
    //materials: &mut ResMut<Assets<StandardMaterial>>,
    //) {
    //let elapsed = self.lifetime.elapsed();
    //let overdue = elapsed > self.emitter_duration;
    //let emitter_elapsed_ms = elapsed.as_millis();
    //let new_emission = (emitter_elapsed_ms / self.delay_between_emission_ms) as i32;

    //if overdue || new_emission <= self.current_emission {
    //return;
    //}

    //self.current_emission = new_emission;
    //let mut rng = thread_rng();
    ////let lifetime = Arc::new(Instant::now());
    //for _ in 0..self.particles_per_emission {
    //let emitter_position = gen_abs_range(&mut rng, self.emitter_diameter);
    //let distortion = gen_dyn_range(&mut rng, self.emission_distortion);
    //let x = (self.position.x + distortion) + emitter_position * self.angle_radians.cos();
    //let y = (self.position.y + distortion) + emitter_position * self.angle_radians.sin();
    //let z = 0.;

    //let diffusion_delta = gen_dyn_range(&mut rng, self.diffusion_radians);
    //let angle_radians = self.angle_emission_radians + diffusion_delta;
    //let vx = self.particle_speed * angle_radians.cos();
    //let vy = self.particle_speed * angle_radians.sin();

    ////let animation_handler = AnimationHandler::new(&self.particle_animation_options);

    //let speed = Speed { vx, vy, vz: 0. };

    //commands
    //.spawn_bundle(PbrBundle {
    //mesh: meshes.add(Mesh::from(shape::Icosphere {
    //radius: self.particle_radius,
    //..Default::default()
    //})),
    //material: materials.add(self.particle_color.into()),
    //transform: Transform {
    //translation: Vec3::new(x, y, z),
    ////scale: Vec3::new(0.1, 0.1, 0.1),
    //..Default::default()
    //},
    //..Default::default()
    //})
    //.insert(speed)
    //.insert(Particle);

    ////self.particles.push(particle);
    //}

    ////self.animate_emitter(emitter_elapsed_ms);
    ////self.update_particles(emitter_elapsed_ms);

    ////if self.particles.is_empty() && overdue {
    ////self.delete = true;
    ////}

    ////self.particle_count = self.particles.len() as u32;
    //}

    //fn update_particles(&mut self, emitter_elapsed_ms: u128) {
    //for i in (0..self.particles.len()).rev() {
    //let mut particle = self.particles.swap_remove(i);

    //let x_force = particle.vx * self.particle_mass;
    //let y_force = particle.vy * self.particle_mass;

    //let x_friction = x_force * self.particle_friction_coefficient;
    //let y_friction = y_force * self.particle_friction_coefficient;

    //let vx = (x_force - x_friction) / self.particle_mass;
    //let vy = (y_force - y_friction) / self.particle_mass;

    ////if let Some(force_handler) = &mut self.force_handler {
    ////let mut data = ForceData {
    ////x: particle.x,
    ////y: particle.y,
    ////vx,
    ////vy,
    ////radius: self.particle_radius,
    ////mass: self.particle_mass,
    ////};

    ////force_handler.apply(&mut data, emitter_elapsed_ms);

    ////particle.vx = data.vx;
    ////particle.vy = data.vy;
    ////} else {
    //particle.vx = vx;
    //particle.vy = vy;
    ////}

    ////let particle_elapsed_ms = particle.lifetime.elapsed().as_millis();

    ////if let Some(animation_handler) = &mut particle.animation_handler {
    ////let mut data: AnimationData = AnimationData {
    ////radius: particle.radius,
    ////color: particle.color,
    ////vx: particle.vx,
    ////vy: particle.vy,
    ////};

    ////animation_handler.animate(&mut data, particle_elapsed_ms);
    ////particle.vx = data.vx;
    ////particle.vy = data.vy;
    ////particle.color = data.color;
    ////particle.radius = data.radius;
    ////}

    ////particle.position.x += particle.vx;
    ////particle.position.y += particle.vy;
    ////particle.position.z += particle.vz;

    ////if let Some(trail_handler) = &mut particle.trail_handler {
    ////let data = TrailData {
    ////radius: particle.radius,
    ////color: particle.color,
    ////x_abs: x,
    ////y_abs: y,
    ////};

    ////trail_handler.animate(&data, particle_elapsed_ms);
    ////}

    ////if let Some(texture) = self.particle_texture {
    ////let side = particle.radius * 2.;
    ////let dest_size = Some(Vec2::new(side, side));

    ////let params = DrawTextureParams {
    ////dest_size,
    ////..Default::default()
    ////};
    ////} else {
    ////}

    ////let diameter = particle.radius * 2.;

    ////if let Some(bounds) = &self.bounds {
    ////let position = &mut particle.position;
    ////if position.x < bounds.start_x
    ////|| bounds.end_x < position.x + diameter
    ////|| position.y < bounds.start_y
    ////|| bounds.end_y < position.y + diameter
    ////|| position.z < bounds.start_z
    ////|| bounds.end_z < position.z + diameter
    ////{
    ////continue; // removes particle.
    ////}
    ////} else if particle_elapsed_ms <= self.particle_lifetime_ms {
    ////self.particles.push(particle);
    ////}
    //}
    //}
}

fn gen_dyn_range(rng: &mut ThreadRng, val: f32) -> f32 {
    if 0. < val {
        rng.gen_range(-val..val)
    } else {
        0.
    }
}

fn gen_abs_range(rng: &mut ThreadRng, val: f32) -> f32 {
    if 0. < val {
        rng.gen_range(0_f32..val)
    } else {
        0.
    }
}
