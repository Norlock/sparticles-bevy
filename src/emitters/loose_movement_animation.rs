use super::emitter_animation::EmitterAnimate;
use crate::{angles::Angles, emitters::emitter_animation::EmitterData};
use rand::{thread_rng, Rng};

pub struct LooseMovementAnimation {
    pub from_ms: u32,
    pub until_ms: u32,
    pub speed: f32,
    pub stray_radians: f32,
    pub angles_radians: Angles,
    // pub max_range: f32,
}

// TODO pass base point which will act as a guideline, the further the emitter will fly off the more force it
// will apply to return. Using inverse gravitational force.
impl EmitterAnimate for LooseMovementAnimation {
    fn animate(&mut self, data: &mut EmitterData, cycle_ms: u32) {
        if cycle_ms < self.from_ms || self.until_ms <= cycle_ms {
            return;
        }

        let stray = thread_rng().gen_range(-self.stray_radians..self.stray_radians);
        //let vx =
        //self.vx = (self.vx * stray.cos()) - (self.vy * stray.sin());
        //self.vy = (self.vx * stray.sin()) + (self.vy * stray.cos());

        //data.x += self.vx;
        //data.y += self.vy;
    }
}
