use crate::math::velocity::equalize_total_speed;
use crate::math::velocity::stray_velocity;
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
        stray_velocity(velocity, self.stray_radians);
    }
}
