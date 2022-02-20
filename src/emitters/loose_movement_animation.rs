use super::emitter_animation::EmitterAnimate;
use crate::Vec3;
use crate::{emitters::emitter_animation::EmitterData, math::velocity::stray_velocity};

pub struct LooseMovementAnimation {
    pub stray_radians: f32,
    pub base: Vec3,
    pub range: f32,
    pub gravitational_force: f32,
    pub friction_coefficient: f32,
    pub base_mass: f32,
    pub emitter_mass: f32,
}

// TODO pass base point which will act as a guideline, the further the emitter will fly off the more force it
// will apply to return. Using inverse gravitational force.
impl EmitterAnimate for LooseMovementAnimation {
    fn animate(&mut self, data: &mut EmitterData, _: u32) {
        stray_velocity(&mut data.velocity, self.stray_radians);
        self.apply_inverse_gravitational_force(data);

        let velocity = &mut data.velocity;
        let x_force = velocity.vx * self.emitter_mass;
        let y_force = velocity.vy * self.emitter_mass;
        let z_force = velocity.vz * self.emitter_mass;

        let friction_multiplier = 1. - self.friction_coefficient;
        velocity.vx = x_force * friction_multiplier / self.emitter_mass;
        velocity.vy = y_force * friction_multiplier / self.emitter_mass;
        velocity.vz = z_force * friction_multiplier / self.emitter_mass;
    }
}

const DEAD_ZONE: f32 = 1.;

impl LooseMovementAnimation {
    fn apply_inverse_gravitational_force(&mut self, data: &mut EmitterData) {
        let position = &data.position;
        let velocity = &mut data.velocity;
        let emitter_size = &mut data.emit_options.emitter_size;

        // TODO emitter can be rotated so center x, y, z might be wrong.
        let emitter_center_x = position.x + emitter_size.length / 2.;
        let emitter_center_y = position.y;
        let emitter_center_z = position.z + emitter_size.depth / 2.;

        let range_delta = self.range - DEAD_ZONE;

        let x_distance = self.base.x - emitter_center_x;
        let x_pull_distance = self.range - x_distance.abs().min(range_delta);

        let y_distance = self.base.y - emitter_center_y;
        let y_pull_distance = self.range - y_distance.abs().min(range_delta);

        let z_distance = self.base.z - emitter_center_z;
        let z_pull_distance = self.range - z_distance.abs().min(range_delta);

        let x_distance_pow = x_pull_distance.powi(2);
        let y_distance_pow = y_pull_distance.powi(2);
        let z_distance_pow = z_pull_distance.powi(2);

        let distance_pow = x_distance_pow + y_distance_pow + z_distance_pow;

        let top_formula = self.gravitational_force * self.base_mass * self.emitter_mass;
        let force = top_formula / distance_pow;

        let x_percentage = 1. - (x_distance_pow / distance_pow);
        let y_percentage = 1. - (y_distance_pow / distance_pow);
        let z_percentage = 1. - (z_distance_pow / distance_pow);

        let vx = force * x_percentage / self.emitter_mass;
        velocity.vx += vx * x_distance.signum() * data.delta_seconds;

        let vy = force * y_percentage / self.emitter_mass;
        velocity.vy += vy * y_distance.signum() * data.delta_seconds;

        let vz = force * z_percentage / self.emitter_mass;
        velocity.vz += vz * z_distance.signum() * data.delta_seconds;

        println!(
            "emc {} base_x {} force {} x_distance {}",
            emitter_center_x, self.base.x, force, x_pull_distance
        );
    }
}
