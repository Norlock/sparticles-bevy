//use crate::animations::animation::AnimationData;
//use crate::animations::animation_handler::AnimationHandler;
//use crate::animations::animation_handler::AnimationOptions;
use crate::emitters::emitter_animation_handler::EmitterAnimationHandler;
use crate::forces::force::ForceData;
use crate::forces::force_handler;
use crate::forces::force_handler::ForceHandler;
use crate::point::Degrees;
use crate::point::Point;
use crate::point::Radians;
use bevy::core::FixedTimestep;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::tasks::TaskPool;
//use crate::trails::trail_animation::TrailData;
//use crate::trails::trail_handler::TrailHandler;
use crate::{position, Position};
use bevy::prelude::*;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::emitter_animation::EmitterData;

#[derive(Debug)]
pub struct EmitterSize {
    pub length: f32,
    pub depth: f32,
}

pub struct EmitterOptions {
    pub emitter_position: Position,
    pub emitter_size: EmitterSize,
    pub emitter_duration: Duration,
    pub angle_degrees: Degrees,
    /// Initial spread factor x,y / z
    pub diffusion_degrees: Degrees,
    pub emission_distortion: f32,
    pub particle_color: Color,
    //pub particle_texture: Option<Texture2D>,
    pub particles_per_emission: u32,
    pub delay_between_emission_ms: u32,
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
    pub force_handler: Option<ForceHandler>,
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
//particle_texture: Option<Texture2D>,
//trail_handler: Option<TrailHandler>,
//particle_animation_options: Option<AnimationOptions>,
//emitter_animation_handler: Option<EmitterAnimationHandler>,
//pub particle_count: u32,
//}

#[derive(Debug, Component)]
struct LifeCycle {
    lifetime: Arc<Instant>,
    duration_ms: u128,
}

#[derive(Debug, Component)]
struct EmitOptions {
    angle_radians: Radians,
    angle_emission_radians: f32,
    diffusion_radians: Radians,
    particles_per_emission: u32,
    delay_between_emission: u32,
    iteration: i32,
    emission_distortion: f32,
    emitter_size: EmitterSize,
}

#[derive(Debug, Component)]
struct ParticleOptions {
    duration_ms: u128,
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

#[derive(Debug, Component)]
pub struct ParticleAttributes {
    radius: f32,
    mass: f32,
    friction_coefficient: f32,
}

pub struct Materials {
    particle_color: Handle<StandardMaterial>,
    particle_mesh: Handle<Mesh>,
}

//trail_handler: Option<TrailHandler>,
//animation_handler: Option<AnimationHandler>,

const EMIT_RADIANS: f32 = 90_f32 * (std::f32::consts::PI / 181.0f32); // 0 deg will be emitting above

pub struct EmitterPlugin;

impl Plugin for EmitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.01667))
                .with_system(transform_particle_system)
                .with_system(update_emitter_system)
                .with_system(apply_forces_system),
        );
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
    }
}

fn apply_forces_system(
    mut particles_query: Query<(&mut Speed, &Transform, &ParticleAttributes), With<Particle>>,
    forces_query: Query<&ForceHandler>,
) {
    for force_handler in forces_query.iter() {
        let elapsed_ms = force_handler.lifetime.elapsed().as_millis();

        for (mut speed, transform, attributes) in particles_query.iter_mut() {
            let mut data = ForceData {
                x: transform.translation.x,
                y: transform.translation.y,
                z: transform.translation.z,
                vx: speed.vx,
                vy: speed.vy,
                vz: speed.vz,
                radius: attributes.radius,
                mass: attributes.radius,
            };

            force_handler.apply(&mut data, elapsed_ms);

            speed.vx = data.vx;
            speed.vy = data.vy;
            speed.vz = data.vz;
        }
    }
}

fn transform_particle_system(
    mut query: Query<
        (
            Entity,
            &mut Speed,
            &mut Transform,
            &LifeCycle,
            &ParticleAttributes,
        ),
        With<Particle>,
    >,
    mut commands: Commands,
) {
    for (entity, mut speed, mut transform, cycle, attributes) in query.iter_mut() {
        let x_force = speed.vx * attributes.mass;
        let y_force = speed.vy * attributes.mass;
        let z_force = speed.vz * attributes.mass;

        let x_friction = x_force * attributes.friction_coefficient;
        let y_friction = y_force * attributes.friction_coefficient;
        let z_friction = z_force * attributes.friction_coefficient;

        speed.vx = (x_force - x_friction) / attributes.mass;
        speed.vy = (y_force - y_friction) / attributes.mass;
        speed.vz = (z_force - z_friction) / attributes.mass;

        transform.translation.x += speed.vx;
        transform.translation.y += speed.vy;
        transform.translation.z += speed.vz;

        if cycle.duration_ms < cycle.lifetime.elapsed().as_millis() {
            commands.entity(entity).despawn();
        }
    }
}

// TODO materials as component for emitter instead of resource.
fn update_emitter_system(
    mut query: Query<
        (
            Entity,
            &LifeCycle,
            &mut EmitOptions,
            &ParticleOptions,
            &Position,
        ),
        With<Emitter>,
    >,
    mut commands: Commands,
    materials: Res<Materials>,
) {
    for (entity, cycle, mut emit_options, particle, position) in query.iter_mut() {
        let elapsed_ms = cycle.lifetime.elapsed().as_millis();
        let overdue = elapsed_ms > cycle.duration_ms;

        if overdue {
            commands.entity(entity).despawn();
            return;
        }

        let new_iteration = elapsed_ms as i32 / emit_options.delay_between_emission as i32;

        if new_iteration == emit_options.iteration {
            return;
        }

        emit_options.iteration = new_iteration;

        let mut rng = thread_rng();

        let lifetime = Arc::new(Instant::now());
        for _ in 0..emit_options.particles_per_emission {
            let emitter_length = gen_abs_range(&mut rng, emit_options.emitter_size.length);
            let emitter_depth = gen_abs_range(&mut rng, emit_options.emitter_size.depth);
            let distortion = gen_dyn_range(&mut rng, emit_options.emission_distortion);

            let Point(elevation, bearing) = emit_options.angle_radians;
            let x = (position.x + distortion) + emitter_length * elevation.cos() * bearing.cos();
            let y = (position.y + distortion) + emitter_length * elevation.sin() * bearing.cos();
            let z = (position.z + distortion) + emitter_depth * bearing.sin();

            let diffusion_elevation_delta =
                gen_dyn_range(&mut rng, emit_options.diffusion_radians.0);
            let bearing_radians = gen_dyn_range(&mut rng, emit_options.diffusion_radians.1);
            let elevation_radians = emit_options.angle_emission_radians + diffusion_elevation_delta;

            let vx = particle.speed * elevation_radians.cos() * bearing_radians.cos();
            let vy = particle.speed * elevation_radians.sin() * bearing_radians.cos();
            let vz = particle.speed * bearing_radians.sin();

            //let animation_handler = AnimationHandler::new(&self.particle_animation_options);
            let bundle = PbrBundle {
                mesh: materials.particle_mesh.clone(),
                material: materials.particle_color.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, z),
                    //scale: Vec3::new(0.1, 0.1, 0.1),
                    ..Default::default()
                },
                ..Default::default()
            };
            let speed = Speed { vx, vy, vz };
            let cycle = LifeCycle {
                lifetime: lifetime.clone(),
                duration_ms: particle.duration_ms,
            };

            let attributes = ParticleAttributes {
                friction_coefficient: particle.friction_coefficient,
                radius: particle.radius,
                mass: particle.mass,
            };

            commands
                .spawn()
                .insert_bundle(bundle)
                .insert_bundle((speed, cycle, attributes, Particle));
        }
    }
}

impl Emitter {
    pub fn create(
        options: EmitterOptions,
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let EmitterOptions {
            emitter_position,
            emitter_size,
            emitter_duration,
            angle_degrees,
            diffusion_degrees,
            emission_distortion,
            particle_color,
            particles_per_emission,
            delay_between_emission_ms: delay_between_emission,
            particle_lifetime,
            particle_radius,
            particle_mass,
            particle_speed,
            particle_friction_coefficient,
            bounds,
            //emitter_animation_handler,
            force_handler,
        } = options;

        let angle_radians = angle_degrees.to_radians();
        let angle_emission_radians = angle_radians.0 + EMIT_RADIANS;

        let emit_options = EmitOptions {
            particles_per_emission,
            diffusion_radians: diffusion_degrees.to_radians(),
            delay_between_emission,
            iteration: -1,
            angle_radians,
            angle_emission_radians,
            emission_distortion,
            emitter_size,
        };

        let emit_time = LifeCycle {
            duration_ms: emitter_duration.as_millis(),
            lifetime: Arc::new(Instant::now()),
        };

        let spawn_options = ParticleOptions {
            speed: particle_speed,
            color: particle_color,
            friction_coefficient: particle_friction_coefficient,
            radius: particle_radius,
            duration_ms: particle_lifetime.as_millis(),
            mass: particle_mass,
        };

        commands.insert_resource(Materials {
            particle_color: materials.add(particle_color.into()),
            particle_mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: particle_radius,
                ..Default::default()
            })),
        });

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

        if let Some(force_handler) = force_handler {
            builder.insert(force_handler);
        }
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
