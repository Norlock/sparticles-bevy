//use crate::animations::animation::AnimationData;
//use crate::animations::animation_handler::AnimationHandler;
//use crate::animations::animation_handler::AnimationOptions;
use crate::animations::animation::Animate;
use crate::animations::animation::AnimationData;
use crate::animations::animation_handler::AnimationHandler;
use crate::animations::animation_handler::AnimationOptions;
use crate::emitters::emitter_animation_handler::EmitterAnimationHandler;
use crate::forces::force::ForceData;
use crate::forces::force_handler::ForceHandler;
use crate::point::Degrees;
use crate::point::Point;
use crate::point::Radians;
use bevy::core::FixedTimestep;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
//use crate::trails::trail_animation::TrailData;
//use crate::trails::trail_handler::TrailHandler;
use crate::Position;
use bevy::prelude::*;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use std::time::Duration;

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
    pub particle_animation_options: Option<AnimationOptions>,
    //pub emitter_animation_handler: Option<EmitterAnimationHandler>,
    pub force_handler: Option<ForceHandler>,
    //pub trail_handler: Option<TrailHandler>,
}

#[derive(Debug, Component)]
pub struct Bounds {
    pub start_x: Option<f32>,
    pub start_y: Option<f32>,
    pub start_z: Option<f32>,
    pub end_x: Option<f32>,
    pub end_y: Option<f32>,
    pub end_z: Option<f32>,
}

#[derive(Debug, Component)]
pub struct Emitter;

#[derive(Debug, Component)]
struct LifeCycle {
    spawned_at: u128,
    duration_ms: u128,
}

impl LifeCycle {
    pub fn elapsed_ms(&self, total_elapsed_ms: u128) -> u128 {
        total_elapsed_ms - self.spawned_at
    }
}

#[derive(Debug, Component)]
struct EmitOptions {
    angle_radians: Radians,
    angle_emission_radians: f32,
    diffusion_radians: Radians,
    particles_per_emission: u32,
    delay_between_emission_ms: u32,
    iteration: i32,
    emission_distortion: f32,
    emitter_size: EmitterSize,
}

/// Which values particles are deployed with.
#[derive(Debug, Component)]
struct EmitterParticleAttributes {
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
struct Particles(Vec<Entity>);

//#[derive(Component)]
//struct Animations(Vec<Box<dyn Animate + Sync + Send>>);

#[derive(Debug, Component)]
struct Velocity {
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

#[derive(Component)]
pub struct Meshes {
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
                .with_system(spawn_particles_system)
                .with_system(apply_forces_system)
                .with_system(apply_animations_system)
                .with_system(remove_particles_system),
        );
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
    }
}

fn apply_forces_system(
    mut particles_query: Query<(&mut Velocity, &Transform, &ParticleAttributes), With<Particle>>,
    emitter_query: Query<(&ForceHandler, &Particles), With<Emitter>>,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();

    for (force_handler, particles) in emitter_query.iter() {
        for &particle_entity in particles.0.iter() {
            if let Ok((mut velocity, transform, attributes)) =
                particles_query.get_mut(particle_entity)
            {
                let mut data = ForceData {
                    x: transform.translation.x,
                    y: transform.translation.y,
                    z: transform.translation.z,
                    vx: velocity.vx,
                    vy: velocity.vy,
                    vz: velocity.vz,
                    radius: attributes.radius * transform.scale.x,
                    mass: attributes.radius,
                };

                force_handler.apply(&mut data, total_elapsed_ms);

                velocity.vx = data.vx;
                velocity.vy = data.vy;
                velocity.vz = data.vz;
            }
        }
    }
}

fn apply_animations_system(
    mut particles_query: Query<
        (
            &mut Velocity,
            &Handle<StandardMaterial>,
            &mut Transform,
            &LifeCycle,
        ),
        With<Particle>,
    >,
    mut emitter_query: Query<(&mut AnimationHandler, &Particles), With<Emitter>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();

    for (mut animation_handler, particles) in emitter_query.iter_mut() {
        for &particle_entity in particles.0.iter() {
            if let Ok((mut velocity, handle, mut transform, life_cycle)) =
                particles_query.get_mut(particle_entity)
            {
                let material = &mut materials.get_mut(handle).unwrap();

                let mut data = AnimationData {
                    color: &mut material.base_color,
                    vx: velocity.vx,
                    vy: velocity.vy,
                    vz: velocity.vz,
                    scale: transform.scale,
                };

                animation_handler.apply(&mut data, life_cycle.elapsed_ms(total_elapsed_ms));

                velocity.vx = data.vx;
                velocity.vy = data.vy;
                velocity.vz = data.vz;
                transform.scale = data.scale;
                //println!("{}", data.color.g());
                // TODO radius
            }
        }
    }
}

fn remove_particles_system(
    mut particles_query: Query<(&Transform, &ParticleAttributes, &LifeCycle), With<Particle>>,
    emitter_query: Query<(Option<&Bounds>, &Particles), With<Emitter>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();

    for (bounds, particles) in emitter_query.iter() {
        for &particle_entity in particles.0.iter() {
            if let Ok((transform, attributes, life_cycle)) =
                particles_query.get_mut(particle_entity)
            {
                if life_cycle.duration_ms < life_cycle.elapsed_ms(total_elapsed_ms) {
                    commands.entity(particle_entity).despawn();
                    continue;
                }

                let translation = &transform.translation;
                let diameter = attributes.radius * 2.;

                if let Some(bounds) = bounds {
                    let below_x = bounds
                        .start_x
                        .map_or(false, |start_x| translation.x < start_x);

                    let below_y = bounds
                        .start_y
                        .map_or(false, |start_y| translation.y < start_y);

                    let below_z = bounds
                        .start_z
                        .map_or(false, |start_z| translation.z < start_z);

                    let above_x = bounds
                        .end_x
                        .map_or(false, |end_x| end_x < translation.x + diameter);

                    let above_y = bounds
                        .end_y
                        .map_or(false, |end_y| end_y < translation.y + diameter);

                    let above_z = bounds
                        .end_z
                        .map_or(false, |end_z| end_z < translation.z + diameter);

                    if below_x || below_y || below_z || above_x || above_y || above_z {
                        commands.entity(particle_entity).despawn();
                        continue;
                    }
                }
            }
        }
    }
}

fn transform_particle_system(
    mut query: Query<(&mut Velocity, &mut Transform, &ParticleAttributes), With<Particle>>,
) {
    for (mut speed, mut transform, attributes) in query.iter_mut() {
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
    }
}

fn spawn_particles_system(
    mut query: Query<
        (
            &LifeCycle,
            &mut EmitOptions,
            &EmitterParticleAttributes,
            &Position,
            &mut Particles,
            &Meshes,
            Entity,
        ),
        With<Emitter>,
    >,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();

    for (
        life_cycle,
        mut emit_options,
        particle_attributes,
        position,
        mut particles,
        meshes,
        entity,
    ) in query.iter_mut()
    {
        let elapsed_ms = life_cycle.elapsed_ms(total_elapsed_ms);
        let out_of_time = life_cycle.duration_ms < elapsed_ms;
        let new_iteration = elapsed_ms as i32 / emit_options.delay_between_emission_ms as i32;

        if out_of_time {
            if particles.0.is_empty() {
                commands.entity(entity).despawn();
            }
            return;
        } else if new_iteration == emit_options.iteration {
            return;
        }

        emit_options.iteration = new_iteration;

        let mut rng = thread_rng();

        for _ in 0..emit_options.particles_per_emission {
            let emitter_length = gen_abs_range(&mut rng, emit_options.emitter_size.length);
            let emitter_depth = gen_abs_range(&mut rng, emit_options.emitter_size.depth);
            let distortion = gen_dyn_range(&mut rng, emit_options.emission_distortion);

            let Point(elevation, bearing) = emit_options.angle_radians;
            let x = (position.x + distortion) + emitter_length * elevation.cos() * bearing.cos();
            let y = (position.y + distortion) + emitter_length * elevation.sin() * bearing.cos();
            let z = (position.z + distortion + emitter_depth) + emitter_length * bearing.sin();

            let diffusion_elevation_delta =
                gen_dyn_range(&mut rng, emit_options.diffusion_radians.0);
            let bearing_radians = gen_dyn_range(&mut rng, emit_options.diffusion_radians.1);
            let elevation_radians = emit_options.angle_emission_radians + diffusion_elevation_delta;

            let vx = particle_attributes.speed * elevation_radians.cos() * bearing_radians.cos();
            let vy = particle_attributes.speed * elevation_radians.sin() * bearing_radians.cos();
            let vz = particle_attributes.speed * bearing_radians.sin();

            let bundle = PbrBundle {
                material: materials.add(particle_attributes.color.into()),
                mesh: meshes.particle_mesh.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, z),
                    ..Default::default()
                },
                ..Default::default()
            };

            let speed = Velocity { vx, vy, vz };
            let life_cycle = LifeCycle {
                spawned_at: total_elapsed_ms,
                duration_ms: particle_attributes.duration_ms,
            };

            let attributes = ParticleAttributes {
                friction_coefficient: particle_attributes.friction_coefficient,
                radius: particle_attributes.radius,
                mass: particle_attributes.mass,
            };

            let id = commands
                .spawn()
                .insert_bundle(bundle)
                .insert_bundle((speed, life_cycle, attributes, Particle))
                .id();

            particles.0.push(id);
        }
    }
}

impl Emitter {
    pub fn create(
        options: EmitterOptions,
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        elapsed_ms: u128,
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
            particle_animation_options,
            force_handler,
        } = options;

        let angle_radians = angle_degrees.to_radians();
        let angle_emission_radians = angle_radians.0 + EMIT_RADIANS;

        let emit_options = EmitOptions {
            particles_per_emission,
            diffusion_radians: diffusion_degrees.to_radians(),
            delay_between_emission_ms: delay_between_emission,
            iteration: -1,
            angle_radians,
            angle_emission_radians,
            emission_distortion,
            emitter_size,
        };

        let emit_time = LifeCycle {
            duration_ms: emitter_duration.as_millis(),
            spawned_at: elapsed_ms,
        };

        let spawn_options = EmitterParticleAttributes {
            speed: particle_speed,
            color: particle_color,
            friction_coefficient: particle_friction_coefficient,
            radius: particle_radius,
            duration_ms: particle_lifetime.as_millis(),
            mass: particle_mass,
        };

        let meshes = Meshes {
            particle_mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: particle_radius,
                ..Default::default()
            })),
        };

        let particles = Particles(Vec::new());

        let mut builder = commands.spawn();
        builder
            .insert(emit_options)
            .insert(emit_time)
            .insert(emitter_position)
            .insert(spawn_options)
            .insert(meshes)
            .insert(particles)
            .insert(Emitter);

        if let Some(bounds) = bounds {
            builder.insert(bounds);
        }

        if let Some(force_handler) = force_handler {
            builder.insert(force_handler);
        }

        if let Some(options) = particle_animation_options {
            builder.insert(AnimationHandler::new(options));
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

    ////let particle_elapsed_ms = particle.lifetime.elapsed().as_millis();

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
