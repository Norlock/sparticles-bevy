use std::time::Duration;
use std::time::Instant;

use crate::point::Point;
use macroquad::prelude::Color;

//
pub struct SwarmEmitter {
    pub boid_speed: f32,
    pub boid_count: u32,
    pub boid_color: Color, // TODO choice between color or texture.
    pub boid_radius: f32,
    pub emission_delay_ms: u128,
    pub diffusion: f32,
    pub flight_pattern: Vec<Point>,
    pub boids: Vec<Boid>, // 1st on is the leader
    lifetime: Instant,
    duration: Duration,
}

pub struct Boid {
    x: f32,
    y: f32,
    angle_degrees: f32,
}
