use super::emitter_animation::EmitterAnimate;
use super::emitter_animation::EmitterData;

pub struct DiffusionAnimation {
    pub from_ms: u32,
    pub until_ms: u32,
    pub start_diffusion_degrees: f32,
    pub end_diffusion_degrees: f32,
}

impl EmitterAnimate for DiffusionAnimation {
    fn animate(&mut self, data: &mut EmitterData, cycle_ms: u32) {
        if cycle_ms < self.from_ms || self.until_ms <= cycle_ms {
            return;
        }

        let delta_current = cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;

        // calculate percent
        let fraction = delta_current as f32 / delta_max as f32;
        let angle_degrees = self.start_diffusion_degrees
            + fraction * (self.end_diffusion_degrees - self.start_diffusion_degrees);

        data.diffusion_radians = angle_degrees.to_radians();
    }
}
