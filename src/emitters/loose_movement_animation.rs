use super::emitter_animation::EmitterAnimate;
use crate::emitters::emitter_animation::EmitterData;
use macroquad::prelude::rand;

pub struct LooseMovementAnimation {
    pub from_ms: u32,
    pub until_ms: u32,
    pub vx: f32,
    pub vy: f32,
    pub stray_radians: f32,
}

impl EmitterAnimate for LooseMovementAnimation {
    fn animate(&mut self, data: &mut EmitterData, cycle_ms: u32) {
        if cycle_ms < self.from_ms || self.until_ms <= cycle_ms {
            return;
        }

        let stray = rand::gen_range(-self.stray_radians, self.stray_radians);
        self.vx = (self.vx * stray.cos()) - (self.vy * stray.sin());
        self.vy = (self.vx * stray.sin()) + (self.vy * stray.cos());

        data.x += self.vx;
        data.y += self.vy;
    }
}
