use super::force::{Force, ForceData};
use bevy::math::Vec3;

pub struct GravitationalForce {
    /// In newton
    pub gravitation_force: f32,
    /// Use to exclude extreme gravitational pulls, e.g. 20.
    pub dead_zone: f32,
    pub mass: f32,
    pub from_ms: u128,
    pub until_ms: u128,
    pub start: Vec3,
    pub end: Vec3,
}

impl GravitationalForce {
    fn current_point(&self, force_cycle_ms: u128) -> Vec3 {
        let delta_current = force_cycle_ms - self.from_ms;
        let delta_end = self.until_ms - self.from_ms;

        let fraction = delta_current as f32 / delta_end as f32;
        Vec3::lerp(self.start, self.end, fraction)
    }
}

impl Force for GravitationalForce {
    // Based on newton's law of universal gravity.
    fn apply(&self, data: &mut ForceData, force_cycle_ms: u128) {
        if force_cycle_ms < self.from_ms || self.until_ms <= force_cycle_ms {
            return;
        }

        let gravitational_point = self.current_point(force_cycle_ms);

        let position = data.position;
        let velocity = &mut data.velocity;

        let particle_center_x = position.x + data.radius.x;
        let particle_center_y = position.y + data.radius.y;
        let particle_center_z = position.z + data.radius.z;
        let x_distance = gravitational_point.x - particle_center_x;
        let y_distance = gravitational_point.y - particle_center_y;
        let z_distance = gravitational_point.z - particle_center_z;

        if x_distance.abs() < self.dead_zone
            && y_distance.abs() < self.dead_zone
            && z_distance.abs() < self.dead_zone
        {
            return;
        }

        let x_distance_pow = x_distance.powi(2);
        let y_distance_pow = y_distance.powi(2);
        let z_distance_pow = z_distance.powi(2);

        let distance_pow = x_distance_pow + y_distance_pow + z_distance_pow;

        let top_formula = self.gravitation_force * self.mass * data.mass;
        let force = top_formula / distance_pow;

        let x_percentage = x_distance_pow / distance_pow;
        let y_percentage = y_distance_pow / distance_pow;
        let z_percentage = z_distance_pow / distance_pow;

        let vx = force * x_percentage / data.mass;
        velocity.vx += vx * x_distance.signum();

        let vy = force * y_percentage / data.mass;
        velocity.vy += vy * y_distance.signum();

        let vz = force * z_percentage / data.mass;
        velocity.vz += vz * z_distance.signum();
    }
}
