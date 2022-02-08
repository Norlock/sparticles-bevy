use crate::force::force::{Force, ForceData};
use crate::point::Point;

pub struct GravitationalForce {
    /// In newton
    pub gravitation_force: f32,
    /// Use to exclude extreme gravitational pulls, e.g. 20.
    pub dead_zone: f32,
    pub mass: f32,
    pub from_ms: u128,
    pub until_ms: u128,
    pub start: Point,
    pub end: Point,
}

impl GravitationalForce {
    fn current_point(&self, force_cycle_ms: u128) -> Point {
        let delta_current = force_cycle_ms - self.from_ms;
        let delta_end = self.until_ms - self.from_ms;

        let fraction = delta_current as f32 / delta_end as f32;

        let x = self.start.0 + fraction * (self.end.0 - self.start.0);
        let y = self.start.1 + fraction * (self.end.1 - self.start.1);

        Point(x, y)
    }
}

impl Force for GravitationalForce {
    // Based on newton's law of universal gravity.
    fn apply(&self, particle: &mut ForceData, force_cycle_ms: u128) {
        if force_cycle_ms < self.from_ms || self.until_ms <= force_cycle_ms {
            return;
        }

        let point = self.current_point(force_cycle_ms);

        let particle_center_x = particle.x + particle.radius;
        let particle_center_y = particle.y + particle.radius;
        let x_distance = point.0 - particle_center_x;
        let y_distance = point.1 - particle_center_y;

        if x_distance.abs() < self.dead_zone && y_distance.abs() < self.dead_zone {
            return;
        }

        let x_distance_pow = x_distance.powi(2);
        let y_distance_pow = y_distance.powi(2);
        let distance_pow = x_distance_pow + y_distance_pow;

        let top_formula = self.gravitation_force * self.mass * particle.mass;
        let force = top_formula / distance_pow;

        let x_percentage = x_distance_pow / distance_pow;
        let y_percentage = y_distance_pow / distance_pow;

        let vx = force * x_percentage / particle.mass;
        if 0. < x_distance {
            particle.vx += vx;
        } else if x_distance < 0. {
            particle.vx -= vx;
        }

        let vy = force * y_percentage / particle.mass;
        if 0. < y_distance {
            particle.vy += vy;
        } else if y_distance < 0. {
            particle.vy -= vy;
        }
    }
}
