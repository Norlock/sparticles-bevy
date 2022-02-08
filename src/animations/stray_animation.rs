use super::animation::Animate;
use super::animation::AnimationData;
use super::animation::AnimationTime;
use macroquad::prelude::rand;

pub struct StrayAnimation {
    from_ms: u32,
    until_ms: u32,
    strayness_radians: f32,
}

impl StrayAnimation {
    pub fn new(from_ms: u32, until_ms: u32, strayness_degrees: f32) -> Self {
        Self {
            from_ms,
            until_ms,
            strayness_radians: strayness_degrees.to_radians(),
        }
    }
}

impl Animate for StrayAnimation {
    fn animate(&self, data: &mut AnimationData, time: &AnimationTime) {
        if time.cycle_ms < self.from_ms || self.until_ms <= time.cycle_ms {
            return;
        }

        let stray = rand::gen_range(-self.strayness_radians, self.strayness_radians);
        data.vx = (data.vx * stray.cos()) - (data.vy * stray.sin());
        data.vy = (data.vx * stray.sin()) + (data.vy * stray.cos());
    }
}
