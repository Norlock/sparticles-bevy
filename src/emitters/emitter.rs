use crate::angles::Angles;
use crate::animations::animation::AnimationData;
use crate::animations::animation_handler::AnimationHandler;
use crate::animations::animation_handler::AnimationOptions;
use crate::emitters::emitter_animation_handler::EmitterAnimationHandler;
use crate::forces::force::ForceData;
use crate::forces::force_handler::ForceHandler;
//use crate::trails::trail_animation::TrailData;
//use crate::trails::trail_handler::TrailHandler;
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
    pub emitter_transform: Transform,
    pub emitter_size: EmitterSize,
    pub emitter_duration: Duration,
    pub emitter_velocity: Velocity,
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
pub struct LifeCycle {
    pub spawned_at: u128,
    pub duration_ms: u128,
    pub iteration: i32,
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

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

impl Velocity {
    pub fn new(vx: f32, vy: f32, vz: f32) -> Self {
        Self { vx, vy, vz }
    }

    pub fn zero() -> Self {
        Default::default()
    }
}

#[derive(Debug, Component)]
pub struct ParticleAttributes {
    pub radius: f32,
    pub mass: f32,
    pub friction_coefficient: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct Materials {
    particle_mesh: Handle<Mesh>,
    particle_material: Handle<StandardMaterial>,
}

const EMIT_RADIANS: f32 = 90_f32 * (std::f32::consts::PI / 180_f32); // 0 deg will be emitting above

pub struct EmitterPlugin;

impl Plugin for EmitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(transform_particle_system)
            .add_system(apply_forces_system)
            .add_system(apply_animations_system)
            .add_system(remove_particles_system)
            .add_system(animate_emitter_system);
    }
}

fn apply_forces_system(
    mut particles_query: Query<
        (&Parent, &mut Velocity, &Transform, &ParticleAttributes),
        With<Particle>,
    >,
    emitter_query: Query<(&ForceHandler, &LifeCycle), With<Emitter>>,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();
    let delta_seconds = time.delta_seconds();

    for (parent, mut velocity, transform, attributes) in particles_query.iter_mut() {
        let (force_handler, life_cycle) = emitter_query.get(parent.0).unwrap();
        let elapsed_ms = life_cycle.elapsed_ms(total_elapsed_ms);

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
            delta_seconds,
        };

        force_handler.apply(&mut data, elapsed_ms);
    }
}

fn apply_animations_system(
    mut particles_query: Query<
        (&Parent, &mut Velocity, &mut Transform, &LifeCycle),
        With<Particle>,
    >,
    mut emitter_query: Query<&mut AnimationHandler, With<Emitter>>,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();

    for (parent, mut velocity, mut transform, life_cycle) in particles_query.iter_mut() {
        let mut animation_handler = emitter_query.get_mut(parent.0).unwrap();
        //let material = &mut materials.get_mut(handle).unwrap();

        let mut color = Color::rgba(0., 0., 0., 1.);
        let mut data = AnimationData {
            color: &mut color,
            scale: &mut transform.scale,
            velocity: &mut velocity,
        };

        animation_handler.apply(&mut data, life_cycle.elapsed_ms(total_elapsed_ms));
    }
}

fn remove_particles_system(
    particles_query: Query<
        (Entity, &Parent, &Transform, &ParticleAttributes, &LifeCycle),
        With<Particle>,
    >,
    emitter_query: Query<Option<&Bounds>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();

    for (entity, parent, transform, attributes, life_cycle) in particles_query.iter() {
        let bounds = emitter_query.get(parent.0).unwrap();

        if life_cycle.duration_ms < life_cycle.elapsed_ms(total_elapsed_ms) {
            commands.entity(parent.0).remove_children(&[entity]);
            commands.entity(entity).despawn();
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
                commands.entity(parent.0).remove_children(&[entity]);
                commands.entity(entity).despawn();
                continue;
            }
        }
    }
}

fn transform_particle_system(
    mut query: Query<(&mut Velocity, &mut Transform, &ParticleAttributes), With<Particle>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (mut velocity, mut transform, attributes) in query.iter_mut() {
        let x_force = velocity.vx * attributes.mass;
        let y_force = velocity.vy * attributes.mass;
        let z_force = velocity.vz * attributes.mass;

        let friction_multiplier = 1. - attributes.friction_coefficient;
        velocity.vx = x_force * friction_multiplier / attributes.mass;
        velocity.vy = y_force * friction_multiplier / attributes.mass;
        velocity.vz = z_force * friction_multiplier / attributes.mass;

        transform.translation.x += velocity.vx * delta;
        transform.translation.y += velocity.vy * delta;
        transform.translation.z += velocity.vz * delta;
    }
}

fn spawn_particles_system(
    mut query: Query<
        (
            &mut LifeCycle,
            &EmitOptions,
            &EmitterParticleAttributes,
            &Materials,
            Option<&Children>,
            Entity,
        ),
        With<Emitter>,
    >,
    mut commands: Commands,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();

    for (mut life_cycle, emit_options, particle_attributes, meshes, children, entity) in
        query.iter_mut()
    {
        let elapsed_ms = life_cycle.elapsed_ms(total_elapsed_ms);
        let out_of_time = life_cycle.duration_ms < elapsed_ms;
        let new_iteration = elapsed_ms as i32 / emit_options.delay_between_emission_ms as i32;

        if out_of_time {
            if let Some(children) = children {
                if children.is_empty() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            continue;
        } else if new_iteration == life_cycle.iteration {
            continue;
        }

        life_cycle.iteration = new_iteration;

        let mut rng = thread_rng();

        for _ in 0..emit_options.particles_per_emission {
            let emitter_length = gen_abs_range(&mut rng, emit_options.emitter_size.length);
            let emitter_depth = gen_abs_range(&mut rng, emit_options.emitter_size.depth);
            let distortion = gen_dyn_range(&mut rng, emit_options.emission_distortion);

            let Angles { elevation, bearing } = emit_options.angle_radians;
            // Used to emit perpendicular of emitter.
            let perpendicular = elevation.cos() * -1.;
            let x = distortion + emitter_length * perpendicular * bearing.cos();
            let y = distortion + emitter_length * elevation.sin() * bearing.cos();
            let z = (distortion + emitter_depth) + emitter_length * bearing.sin();

            let diffusion_elevation_delta =
                gen_dyn_range(&mut rng, emit_options.diffusion_radians.elevation);
            let bearing_radians = gen_dyn_range(&mut rng, emit_options.diffusion_radians.bearing);
            let elevation_radians =
                emit_options.angle_emission_radians() + diffusion_elevation_delta;

            // Used to emit perpendicular of emitter.
            let perpendicular = elevation_radians.cos() * -1.;
            let vx = particle_attributes.speed * perpendicular * bearing_radians.cos();
            let vy = particle_attributes.speed * elevation_radians.sin() * bearing_radians.cos();
            let vz = particle_attributes.speed * bearing_radians.sin();

            let pbr_bundle = PbrBundle {
                material: meshes.particle_material.clone(),
                mesh: meshes.particle_mesh.clone(),
                transform: Transform::from_xyz(x, y, z),
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
                color: particle_attributes.color,
            };

            commands
                .spawn()
                .insert_bundle(pbr_bundle)
                .insert(Parent(entity))
                .insert_bundle((speed, life_cycle, attributes, Particle))
                .id();
        }
    }
}

fn animate_emitter_system(
    mut query: Query<
        (
            &LifeCycle,
            &mut EmitOptions,
            &mut EmitterParticleAttributes,
            &mut Transform,
            &mut Velocity,
            &mut EmitterAnimationHandler,
        ),
        With<Emitter>,
    >,
    time: Res<Time>,
) {
    let total_elapsed_ms = time.time_since_startup().as_millis();
    let delta_seconds = time.delta_seconds();

    for (
        life_cycle,
        mut emit_options,
        mut particle_attr,
        mut transform,
        mut velocity,
        mut anim_handler,
    ) in query.iter_mut()
    {
        let mut data = EmitterData {
            particle_attributes: &mut particle_attr,
            emit_options: &mut emit_options,
            transform: &mut transform,
            velocity: &mut velocity,
            delta_seconds,
        };

        anim_handler.animate(&mut data, life_cycle.elapsed_ms(total_elapsed_ms));

        let translation = &mut transform.translation;
        translation.x += velocity.vx * delta_seconds;
        translation.y += velocity.vy * delta_seconds;
        translation.z += velocity.vz * delta_seconds;
    }
}

impl Emitter {
    pub fn create(
        options: EmitterOptions,
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        elapsed_ms: u128,
    ) {
        let EmitterOptions {
            emitter_transform,
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
            emitter_velocity,
        } = options;

        let emit_options = EmitOptions {
            particles_per_emission,
            diffusion_radians: diffusion_degrees.to_radians(),
            angle_radians: angle_degrees.to_radians(),
            delay_between_emission_ms,
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

        let meshes = Materials {
            particle_mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: particle_radius,
                ..Default::default()
            })),
            particle_material: materials.add(StandardMaterial {
                base_color: particle_color,
                //alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
        };

        let pbr_bundle = PbrBundle {
            transform: emitter_transform,
            ..Default::default()
        };

        let mut builder = commands.spawn();
        builder
            .insert(emit_options)
            .insert(emit_time)
            .insert(spawn_options)
            .insert(meshes)
            .insert(emitter_velocity)
            .insert_bundle(pbr_bundle)
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

pub fn gen_dyn_range(rng: &mut ThreadRng, val: f32) -> f32 {
    if 0. < val {
        rng.gen_range(-val..val)
    } else {
        0.
    }
}

pub fn gen_abs_range(rng: &mut ThreadRng, val: f32) -> f32 {
    if 0. < val {
        rng.gen_range(0_f32..val)
    } else {
        0.
    }
}
