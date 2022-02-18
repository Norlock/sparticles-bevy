use super::emitter_animation::EmitterAnimate;
use super::emitter_animation::EmitterData;

pub struct DiffusionAnimation {
    pub from_ms: u32,
    pub until_ms: u32,
    pub start_elevation_radians: f32,
    pub end_elevation_radians: f32,
    pub start_bearing_radians: f32,
    pub end_bearing_radians: f32,
}

impl EmitterAnimate for DiffusionAnimation {
    fn animate(&mut self, data: &mut EmitterData, cycle_ms: u32) {
        if cycle_ms < self.from_ms || self.until_ms <= cycle_ms {
            return;
        }

        let diffusion = &mut data.emit_options.diffusion_radians;
        let delta_current = cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;

        // calculate percent
        let fraction = delta_current as f32 / delta_max as f32;
        diffusion.elevation = self.start_elevation_radians
            + fraction * (self.end_elevation_radians - self.start_elevation_radians);
        diffusion.bearing = self.start_bearing_radians
            + fraction * (self.end_bearing_radians - self.start_bearing_radians);
    }
}
