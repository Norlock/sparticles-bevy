//use crate::animations::animation::AnimationData;
//use crate::animations::animation_handler::AnimationHandler;
//use crate::animations::animation_handler::AnimationOptions;
use crate::animations::animation::AnimationData;
use crate::animations::animation_handler::AnimationHandler;
use crate::animations::animation_handler::AnimationOptions;
use crate::emitters::emitter_animation_handler::EmitterAnimationHandler;
use crate::forces::force::ForceData;
use crate::forces::force_handler::ForceHandler;
use crate::point::Angles;
use bevy::core::FixedTimestep;
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
    pub angle_degrees: Angles,
    /// Initial spread factor x,y / z
    pub diffusion_degrees: Angles,
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
    pub emitter_animation_handler: Option<EmitterAnimationHandler>,
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
    iteration: i32,
}

impl LifeCycle {
    pub fn elapsed_ms(&self, total_elapsed_ms: u128) -> u128 {
        total_elapsed_ms - self.spawned_at
    }
}

#[derive(Debug, Component)]
pub struct EmitOptions {
    pub angle_radians: Angles,
    pub diffusion_radians: Angles,
    pub particles_per_emission: u32,
    pub delay_between_emission_ms: u32,
    pub emission_distortion: f32,
    pub emitter_size: EmitterSize,
}

impl EmitOptions {
    pub fn angle_emission_radians(&self) -> f32 {
        self.angle_radians.elevation + EMIT_RADIANS
    }
}

/// Which values particles are deployed with.
#[derive(Debug, Component)]
pub struct EmitterParticleAttributes {
    pub duration_ms: u128,
    pub radius: f32,
    pub mass: f32,
    pub speed: f32,
    pub friction_coefficient: f32,
    pub color: Color,
}

#[derive(Debug, Component)]
pub struct Particle;

#[derive(Debug, Component)]
struct Particles(Vec<Entity>);

//#[derive(Component)]
//struct Animations(Vec<Box<dyn Animate + Sync + Send>>);

#[derive(Component)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
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
                .with_system(remove_particles_system)
                .with_system(animate_emitter_system),
        );
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
    }
}

fn apply_forces_system(
    mut particles_query: Query<(&mut Velocity, &Transform, &ParticleAttributes), With<Particle>>,
    emitter_query: Query<(&ForceHandler, &Particles, &LifeCycle), With<Emitter>>,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();

    for (force_handler, particles, life_cycle) in emitter_query.iter() {
        let elapsed_ms = life_cycle.elapsed_ms(total_elapsed_ms);

        for &particle_entity in particles.0.iter() {
            if let Ok((mut velocity, transform, attributes)) =
                particles_query.get_mut(particle_entity)
            {
                let scale = &transform.scale;
                let radius = Vec3::new(
                    scale.x * attributes.radius,
                    scale.y * attributes.radius,
                    scale.z * attributes.radius,
                );

                let mut data = ForceData {
                    position: &transform.translation,
                    velocity: &mut velocity,
                    radius,
                    mass: attributes.mass,
                };

                force_handler.apply(&mut data, elapsed_ms);
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
                    scale: &mut transform.scale,
                    velocity: &mut velocity,
                };

                animation_handler.apply(&mut data, life_cycle.elapsed_ms(total_elapsed_ms));
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
    time: Res<Time>,
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

        let delta = time.delta_seconds();
        transform.translation.x += speed.vx * delta;
        transform.translation.y += speed.vy * delta;
        transform.translation.z += speed.vz * delta;
    }
}

fn spawn_particles_system(
    mut query: Query<
        (
            &mut LifeCycle,
            &EmitOptions,
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
        mut life_cycle,
        emit_options,
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
        } else if new_iteration == life_cycle.iteration {
            return;
        }

        life_cycle.iteration = new_iteration;

        let mut rng = thread_rng();

        for _ in 0..emit_options.particles_per_emission {
            let emitter_length = gen_abs_range(&mut rng, emit_options.emitter_size.length);
            let emitter_depth = gen_abs_range(&mut rng, emit_options.emitter_size.depth);
            let distortion = gen_dyn_range(&mut rng, emit_options.emission_distortion);

            let Angles { elevation, bearing } = emit_options.angle_radians;
            let x = (position.x + distortion) + emitter_length * elevation.cos() * bearing.cos();
            let y = (position.y + distortion) + emitter_length * elevation.sin() * bearing.cos();
            let z = (position.z + distortion + emitter_depth) + emitter_length * bearing.sin();

            let diffusion_elevation_delta =
                gen_dyn_range(&mut rng, emit_options.diffusion_radians.elevation);
            let bearing_radians = gen_dyn_range(&mut rng, emit_options.diffusion_radians.bearing);
            let elevation_radians =
                emit_options.angle_emission_radians() + diffusion_elevation_delta;

            let vx = particle_attributes.speed * elevation_radians.cos() * bearing_radians.cos();
            let vy = particle_attributes.speed * elevation_radians.sin() * bearing_radians.cos();
            let vz = particle_attributes.speed * bearing_radians.sin();

            let bundle = PbrBundle {
                material: materials.add(StandardMaterial {
                    base_color: particle_attributes.color,
                    alpha_mode: AlphaMode::Blend,
                    ..Default::default()
                }),
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
                iteration: -1,
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

fn animate_emitter_system(
    mut query: Query<
        (
            &LifeCycle,
            &mut EmitOptions,
            &mut EmitterParticleAttributes,
            &mut Position,
            &mut EmitterAnimationHandler,
        ),
        With<Emitter>,
    >,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();

    for (life_cycle, mut emit_options, mut particle_attr, mut position, mut anim_handler) in
        query.iter_mut()
    {
        let mut data = EmitterData {
            particle_attributes: &mut particle_attr,
            emit_options: &mut emit_options,
            position: &mut position,
        };

        anim_handler.animate(&mut data, life_cycle.elapsed_ms(total_elapsed_ms));
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
            delay_between_emission_ms,
            particle_lifetime,
            particle_radius,
            particle_mass,
            particle_speed,
            particle_friction_coefficient,
            bounds,
            particle_animation_options,
            emitter_animation_handler,
            force_handler,
        } = options;

        let angle_radians = angle_degrees.to_radians();

        let emit_options = EmitOptions {
            particles_per_emission,
            diffusion_radians: diffusion_degrees.to_radians(),
            delay_between_emission_ms,
            angle_radians,
            emission_distortion,
            emitter_size,
        };

        let emit_time = LifeCycle {
            duration_ms: emitter_duration.as_millis(),
            spawned_at: elapsed_ms,
            iteration: -1,
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

        if let Some(ah) = emitter_animation_handler {
            builder.insert(ah);
        }
    }

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
