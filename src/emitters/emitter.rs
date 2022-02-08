use crate::animations::animation::AnimationData;
use crate::animations::animation_handler::AnimationHandler;
use crate::animations::animation_handler::AnimationOptions;
use crate::emitters::emitter_animation_handler::EmitterAnimationHandler;
use crate::force::force::ForceData;
use crate::force::force_handler::ForceHandler;
use crate::trails::trail_animation::TrailData;
use crate::trails::trail_handler::TrailHandler;
use crate::Position;
use macroquad::prelude::*;
use std::rc::Rc;
use std::time::{Duration, Instant};

use super::emitter_animation::EmitterData;

pub struct EmitterOptions {
    pub emitter_position: Position,
    pub emitter_diameter: f32,
    pub emitter_duration: Duration,
    pub angle_degrees: f32,
    /// Initial spread factor
    pub diffusion_degrees: f32,
    pub emission_distortion_px: f32,
    pub particle_color: Color,
    pub particle_texture: Option<Texture2D>,
    pub particles_per_emission: u32,
    pub delay_between_emission: Duration,
    pub particle_lifetime: Duration,
    pub particle_radius: f32,
    pub particle_mass: f32,

    /// Newton force
    pub particle_speed: f32,
    /// number between 0 and 1, e.g. 0.001
    pub particle_friction_coefficient: f32,
    pub respect_grid_bounds: bool,
    pub particle_animation_options: Option<AnimationOptions>,
    pub emitter_animation_handler: Option<EmitterAnimationHandler>,
    pub force_handler: Option<ForceHandler>,
    pub trail_handler: Option<TrailHandler>,
}

#[derive(Debug)]
pub struct Emitter {
    emitter_diameter: f32,
    x: f32,
    y: f32,
    grid_position: Position,
    respect_grid_bounds: bool,
    angle_radians: f32,
    angle_emission_radians: f32,
    diffusion_radians: f32,
    particle_color: Color,
    particle_texture: Option<Texture2D>,
    particles_per_emission: u32,
    delay_between_emission_ms: u128,
    emission_distortion: f32,
    current_emission: i32,
    particle_lifetime_ms: u128,
    particle_radius: f32,
    particle_mass: f32,
    particle_speed: f32,
    particle_friction_coefficient: f32,
    trail_handler: Option<TrailHandler>,
    particles: Vec<EmittedParticle>,
    lifetime: Instant,
    emitter_duration: Duration,
    particle_animation_options: Option<AnimationOptions>,
    force_handler: Option<ForceHandler>,
    emitter_animation_handler: Option<EmitterAnimationHandler>,
    pub delete: bool,
    pub particle_count: u32,
}

#[derive(Debug)]
struct EmittedParticle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    lifetime: Rc<Instant>,
    color: Color,
    trail_handler: Option<TrailHandler>,
    animation_handler: Option<AnimationHandler>,
}

const INVERSE_RADIANS: f32 = -90_f32 * (std::f32::consts::PI / 181.0f32); // 0 deg will be emitting above

impl Emitter {
    pub fn new(grid_position: Position, options: EmitterOptions) -> Self {
        let EmitterOptions {
            emitter_position,
            emitter_diameter,
            emitter_duration,
            angle_degrees,
            diffusion_degrees,
            emission_distortion_px,
            particle_color,
            particle_texture,
            particles_per_emission,
            delay_between_emission,
            particle_lifetime,
            particle_radius,
            particle_mass,
            particle_speed,
            particle_friction_coefficient,
            respect_grid_bounds,
            particle_animation_options,
            emitter_animation_handler,
            force_handler,
            trail_handler,
        } = options;

        let angle_radians = angle_degrees.to_radians();
        let angle_emission_radians = angle_radians + INVERSE_RADIANS;
        let x = emitter_position.x;
        let y = emitter_position.y;

        Self {
            particles_per_emission,
            particles: Vec::new(),
            particle_color,
            particle_texture,
            diffusion_radians: diffusion_degrees.to_radians(),
            particle_mass,
            particle_radius,
            x,
            y,
            grid_position,
            angle_radians,
            angle_emission_radians,
            emission_distortion: emission_distortion_px,
            particle_lifetime_ms: particle_lifetime.as_millis(),
            particle_count: 0,
            emitter_diameter,
            emitter_duration,
            lifetime: Instant::now(),
            current_emission: -1,
            delay_between_emission_ms: delay_between_emission.as_millis(),
            respect_grid_bounds,
            particle_friction_coefficient,
            particle_speed,
            particle_animation_options,
            emitter_animation_handler,
            force_handler,
            trail_handler,
            delete: false,
        }
    }

    fn animate_emitter(&mut self, elapsed_ms: u128) {
        if let Some(anim_handler) = &mut self.emitter_animation_handler {
            let mut data = EmitterData {
                delay_between_emission_ms: self.delay_between_emission_ms,
                particle_speed: self.particle_speed,
                particle_friction_coefficient: self.particle_friction_coefficient,
                particles_per_emission: self.particles_per_emission,
                respect_grid_bounds: self.respect_grid_bounds,
                emitter_diameter: self.emitter_diameter,
                particle_color: self.particle_color,
                emission_distortion: self.emission_distortion,
                angle_radians: self.angle_radians,
                diffusion_radians: self.diffusion_radians,
                particle_radius: self.particle_radius,
                x: self.x,
                y: self.y,
            };

            anim_handler.animate(&mut data, elapsed_ms);

            self.angle_emission_radians = data.angle_radians + INVERSE_RADIANS;
            self.diffusion_radians = data.diffusion_radians;
            self.x = data.x;
            self.y = data.y;
            self.particle_color = data.particle_color;
            self.particle_speed = data.particle_speed;
            self.particle_radius = data.particle_radius;
        }
    }

    pub fn emit(&mut self) {
        let elapsed = self.lifetime.elapsed();
        let overdue = elapsed > self.emitter_duration;
        let emitter_elapsed_ms = elapsed.as_millis();
        let new_emission = (emitter_elapsed_ms / self.delay_between_emission_ms) as i32;

        if !overdue && self.current_emission < new_emission {
            self.current_emission = new_emission;
            let lifetime = Rc::new(Instant::now());
            for _ in 0..self.particles_per_emission {
                self.particles
                    .push(self.create_particle(Rc::clone(&lifetime)));
            }
        }

        self.animate_emitter(emitter_elapsed_ms);
        self.update_particles(emitter_elapsed_ms);

        if self.particles.is_empty() && overdue {
            self.delete = true;
        }
        self.particle_count = self.particles.len() as u32;
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

            if let Some(force_handler) = &mut self.force_handler {
                let mut data = ForceData {
                    x: particle.x,
                    y: particle.y,
                    vx,
                    vy,
                    radius: self.particle_radius,
                    mass: self.particle_mass,
                };

                force_handler.apply(&mut data, emitter_elapsed_ms);

                particle.vx = data.vx;
                particle.vy = data.vy;
            } else {
                particle.vx = vx;
                particle.vy = vy;
            }

            let particle_elapsed_ms = particle.lifetime.elapsed().as_millis();

            if let Some(animation_handler) = &mut particle.animation_handler {
                let mut data: AnimationData = AnimationData {
                    radius: particle.radius,
                    color: particle.color,
                    vx: particle.vx,
                    vy: particle.vy,
                };

                animation_handler.animate(&mut data, particle_elapsed_ms);
                particle.vx = data.vx;
                particle.vy = data.vy;
                particle.color = data.color;
                particle.radius = data.radius;
            }

            particle.x += particle.vx;
            particle.y += particle.vy;

            let x = particle.x + self.grid_position.x;
            let y = particle.y + self.grid_position.y;

            if let Some(trail_handler) = &mut particle.trail_handler {
                let data = TrailData {
                    radius: particle.radius,
                    color: particle.color,
                    x_abs: x,
                    y_abs: y,
                };

                trail_handler.animate(&data, particle_elapsed_ms);
            }

            if let Some(texture) = self.particle_texture {
                let side = particle.radius * 2.;
                let dest_size = Some(Vec2::new(side, side));

                let params = DrawTextureParams {
                    dest_size,
                    ..Default::default()
                };

                draw_texture_ex(texture, x, y, particle.color, params);
            } else {
                draw_circle(x, y, particle.radius, particle.color);
            }

            let diameter = particle.radius * 2.;

            if self.respect_grid_bounds
                && (particle.x < 0.
                    || self.grid_position.width < particle.x + diameter
                    || particle.y < 0.
                    || self.grid_position.height < particle.y + diameter)
            {
                continue; // removes particle.
            } else if particle_elapsed_ms <= self.particle_lifetime_ms {
                self.particles.push(particle);
            }
        }
    }

    fn create_particle(&self, lifetime: Rc<Instant>) -> EmittedParticle {
        let position = rand::gen_range(0., self.emitter_diameter);
        let distortion = rand::gen_range(-self.emission_distortion, self.emission_distortion);
        let x = (self.x + distortion) + position * self.angle_radians.cos();
        let y = (self.y + distortion) + position * self.angle_radians.sin();

        let diffusion_delta = rand::gen_range(-self.diffusion_radians, self.diffusion_radians);

        let angle_radians = self.angle_emission_radians + diffusion_delta;
        let vx = self.particle_speed * angle_radians.cos();
        let vy = self.particle_speed * angle_radians.sin();

        let animation_handler = AnimationHandler::new(&self.particle_animation_options);

        EmittedParticle {
            x,
            y,
            vx,
            vy,
            lifetime,
            radius: self.particle_radius,
            color: self.particle_color,
            trail_handler: self.trail_handler.clone(),
            animation_handler,
        }
    }
}
