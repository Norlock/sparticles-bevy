use rand::thread_rng;
use rand::Rng;

use crate::emitters::emitter::Velocity;

use super::animation::Animate;
use super::animation::AnimationData;
use super::animation::AnimationTime;

pub struct StrayAnimation {
    from_ms: u32,
    until_ms: u32,
    stray_radians: f32,
}

impl StrayAnimation {
    /// Between 1 and 50 for strayness_number is advised
    pub fn new(from_ms: u32, until_ms: u32, strayness_number: f32) -> Self {
        Self {
            from_ms,
            until_ms,
            stray_radians: strayness_number.to_radians(),
        }
    }
}

impl Animate for StrayAnimation {
    fn animate(&self, data: &mut AnimationData, time: &AnimationTime) {
        if time.cycle_ms < self.from_ms || self.until_ms <= time.cycle_ms {
            return;
        }

        let velocity = &mut data.velocity;

        let speed_squared = velocity.vy.powi(2) + velocity.vz.powi(2) + velocity.vx.powi(2);
        let speed = speed_squared.sqrt();

        let mut rng = thread_rng();
        let mut stray_factor = || rng.gen_range(-self.stray_radians..self.stray_radians) / 2.;

        let cos_x = velocity.vx / speed + stray_factor();
        let cos_y = velocity.vy / speed + stray_factor();
        let cos_z = velocity.vz / speed + stray_factor();

        let new_vx = speed * cos_x;
        let new_vy = speed * cos_y;
        let new_vz = speed * cos_z;

        let mut new_velocity = Velocity::new(new_vx, new_vy, new_vz);

        equalize_total_speed(speed_squared, &mut new_velocity);

        velocity.vx = new_velocity.vx;
        velocity.vy = new_velocity.vy;
        velocity.vz = new_velocity.vz;
    }
}

fn equalize_total_speed(speed_squared: f32, new: &mut Velocity) {
    let new_vx_squared = new.vx.powi(2);
    let new_vy_squared = new.vy.powi(2);
    let new_vz_squared = new.vz.powi(2);
    let new_speed_squared = new_vx_squared + new_vy_squared + new_vz_squared;

    let scale_factor = speed_squared / new_speed_squared;
    new.vx = (new_vx_squared * scale_factor).sqrt() * new.vx.signum();
    new.vy = (new_vy_squared * scale_factor).sqrt() * new.vy.signum();
    new.vz = (new_vz_squared * scale_factor).sqrt() * new.vz.signum();
}
