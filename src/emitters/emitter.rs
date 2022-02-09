//use crate::animations::animation::AnimationData;
//use crate::animations::animation_handler::AnimationHandler;
//use crate::animations::animation_handler::AnimationOptions;
use crate::emitters::emitter_animation_handler::EmitterAnimationHandler;
use crate::force::force::ForceData;
use crate::force::force_handler::ForceHandler;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
//use crate::trails::trail_animation::TrailData;
//use crate::trails::trail_handler::TrailHandler;
use crate::{position, Position};
use bevy::{prelude::*, transform};
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

#[derive(Debug)]
pub struct Bounds {
    start_x: f32,
    start_y: f32,
    start_z: f32,
    end_x: f32,
    end_y: f32,
    end_z: f32,
}

#[derive(Debug, Component)]
pub struct Emitter {
    emitter_diameter: f32,
    position: Position,
    bounds: Option<Bounds>,
    angle_radians: f32,
    angle_emission_radians: f32,
    diffusion_radians: f32,
    particle_color: Color,
    //particle_texture: Option<Texture2D>,
    particles_per_emission: u32,
    delay_between_emission_ms: u128,
    emission_distortion: f32,
    current_emission: i32,
    particle_lifetime_ms: u128,
    particle_radius: f32,
    particle_mass: f32,
    particle_speed: f32,
    particle_friction_coefficient: f32,
    //trail_handler: Option<TrailHandler>,
    particles: Vec<EmittedParticle>,
    lifetime: Instant,
    emitter_duration: Duration,
    //particle_animation_options: Option<AnimationOptions>,
    //force_handler: Option<ForceHandler>,
    //emitter_animation_handler: Option<EmitterAnimationHandler>,
    pub delete: bool,
    pub particle_count: u32,
}

#[derive(Debug, Component)]
struct EmittedParticle {
    position: Position,
    vx: f32,
    vy: f32,
    vz: f32,
    radius: f32,
    //lifetime: Arc<Instant>,
    color: Color,
    //trail_handler: Option<TrailHandler>,
    //animation_handler: Option<AnimationHandler>,
}

const EMIT_RADIANS: f32 = 90_f32 * (std::f32::consts::PI / 181.0f32); // 0 deg will be emitting above

fn update_emitter_system(
    mut query: Query<&mut Emitter>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for mut emitter in query.iter_mut() {
        emitter.emit(&mut commands, &mut meshes, &mut materials);
        //println!("{:?}", emitter.particles.len());
    }
}

fn transform_particle_system(mut query: Query<(&mut EmittedParticle, &mut Transform)>) {
    //println!("{}", query.iter().len());
    for (mut particle, mut transform) in query.iter_mut() {
        let vx = particle.vx;
        let vy = particle.vy;
        let vz = particle.vz;

        particle.position.update(vx, vy, vz);
        transform.translation.x = particle.position.x;
        transform.translation.y = particle.position.y;
        transform.translation.z = particle.position.z;
    }
}

pub struct EmitterPlugin;

impl Plugin for EmitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_emitter_system)
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_system(transform_particle_system);
    }
}

impl Emitter {
    pub fn new(options: EmitterOptions) -> Self {
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

        Self {
            particles_per_emission,
            particles: Vec::new(),
            particle_color,
            diffusion_radians: diffusion_degrees.to_radians(),
            particle_mass,
            particle_radius,
            position: emitter_position,
            angle_radians,
            angle_emission_radians,
            emission_distortion,
            particle_lifetime_ms: particle_lifetime.as_millis(),
            particle_count: 0,
            emitter_diameter,
            emitter_duration,
            lifetime: Instant::now(),
            current_emission: -1,
            delay_between_emission_ms: delay_between_emission.as_millis(),
            bounds,
            particle_friction_coefficient,
            particle_speed,
            //emitter_animation_handler,
            //force_handler,
            delete: false,
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

    pub fn emit(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let elapsed = self.lifetime.elapsed();
        let overdue = elapsed > self.emitter_duration;
        let emitter_elapsed_ms = elapsed.as_millis();
        let new_emission = (emitter_elapsed_ms / self.delay_between_emission_ms) as i32;
        let mut rng = thread_rng();

        if !overdue && self.current_emission < new_emission {
            self.current_emission = new_emission;
            //let lifetime = Arc::new(Instant::now());
            for _ in 0..self.particles_per_emission {
                let particle = self.create_particle(&mut rng);
                //println!("{} {}", particle.x, particle.y);
                let position = &particle.position;
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere {
                            radius: particle.radius,
                            ..Default::default()
                        })),
                        material: materials.add(particle.color.into()),
                        transform: Transform {
                            translation: Vec3::new(position.x, position.y, position.z),
                            //scale: Vec3::new(0.1, 0.1, 0.1),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(particle);

                //self.particles.push(particle);
            }
        }

        //self.animate_emitter(emitter_elapsed_ms);
        //self.update_particles(emitter_elapsed_ms);

        //if self.particles.is_empty() && overdue {
        //self.delete = true;
        //}

        //self.particle_count = self.particles.len() as u32;
    }

    fn update_particles(&mut self, emitter_elapsed_ms: u128) {
        for i in (0..self.particles.len()).rev() {
            let mut particle = self.particles.swap_remove(i);

            let x_force = particle.vx * self.particle_mass;
            let y_force = particle.vy * self.particle_mass;

            let x_friction = x_force * self.particle_friction_coefficient;
            let y_friction = y_force * self.particle_friction_coefficient;

            let vx = (x_force - x_friction) / self.particle_mass;
            let vy = (y_force - y_friction) / self.particle_mass;

            //if let Some(force_handler) = &mut self.force_handler {
            //let mut data = ForceData {
            //x: particle.x,
            //y: particle.y,
            //vx,
            //vy,
            //radius: self.particle_radius,
            //mass: self.particle_mass,
            //};

            //force_handler.apply(&mut data, emitter_elapsed_ms);

            //particle.vx = data.vx;
            //particle.vy = data.vy;
            //} else {
            particle.vx = vx;
            particle.vy = vy;
            //}

            //let particle_elapsed_ms = particle.lifetime.elapsed().as_millis();

            //if let Some(animation_handler) = &mut particle.animation_handler {
            //let mut data: AnimationData = AnimationData {
            //radius: particle.radius,
            //color: particle.color,
            //vx: particle.vx,
            //vy: particle.vy,
            //};

            //animation_handler.animate(&mut data, particle_elapsed_ms);
            //particle.vx = data.vx;
            //particle.vy = data.vy;
            //particle.color = data.color;
            //particle.radius = data.radius;
            //}

            particle.position.x += particle.vx;
            particle.position.y += particle.vy;
            particle.position.z += particle.vz;

            //if let Some(trail_handler) = &mut particle.trail_handler {
            //let data = TrailData {
            //radius: particle.radius,
            //color: particle.color,
            //x_abs: x,
            //y_abs: y,
            //};

            //trail_handler.animate(&data, particle_elapsed_ms);
            //}

            //if let Some(texture) = self.particle_texture {
            //let side = particle.radius * 2.;
            //let dest_size = Some(Vec2::new(side, side));

            //let params = DrawTextureParams {
            //dest_size,
            //..Default::default()
            //};
            //} else {
            //}

            let diameter = particle.radius * 2.;

            //if let Some(bounds) = &self.bounds {
            //let position = &mut particle.position;
            //if position.x < bounds.start_x
            //|| bounds.end_x < position.x + diameter
            //|| position.y < bounds.start_y
            //|| bounds.end_y < position.y + diameter
            //|| position.z < bounds.start_z
            //|| bounds.end_z < position.z + diameter
            //{
            //continue; // removes particle.
            //}
            //} else if particle_elapsed_ms <= self.particle_lifetime_ms {
            //self.particles.push(particle);
            //}
        }
    }

    fn create_particle(&self, rng: &mut ThreadRng) -> EmittedParticle {
        let emitter_position = gen_abs_range(rng, self.emitter_diameter);
        let distortion = gen_dyn_range(rng, self.emission_distortion);
        let x = (self.position.x + distortion) + emitter_position * self.angle_radians.cos();
        let y = (self.position.y + distortion) + emitter_position * self.angle_radians.sin();
        let z = 0.;

        let position = Position::new(x, y, z);

        let diffusion_delta = gen_dyn_range(rng, self.diffusion_radians);
        let angle_radians = self.angle_emission_radians + diffusion_delta;
        let vx = self.particle_speed * angle_radians.cos();
        let vy = self.particle_speed * angle_radians.sin();

        //let animation_handler = AnimationHandler::new(&self.particle_animation_options);

        EmittedParticle {
            position,
            vx,
            vy,
            vz: 0.,
            radius: self.particle_radius,
            color: self.particle_color,
            //trail_handler: self.trail_handler.clone(),
            //animation_handler,
        }
    }
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
