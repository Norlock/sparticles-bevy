use super::emitter_animation::EmitterAnimate;
use super::emitter_animation::EmitterData;

pub struct EmitSpeedAnimation {
    pub from_ms: u32,
    pub until_ms: u32,
    pub from_speed: f32,
    pub to_speed: f32,
}

impl EmitterAnimate for EmitSpeedAnimation {
    fn animate(&mut self, data: &mut EmitterData, cycle_ms: u32) {
        if cycle_ms < self.from_ms || self.until_ms <= cycle_ms {
            return;
        }

        let delta_current = cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;

        // calculate percent from 0..1
        let fraction = delta_current as f32 / delta_max as f32;

        data.particle_speed = self.from_speed + fraction * (self.to_speed - self.from_speed);
    }
}
