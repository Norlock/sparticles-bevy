use super::emitter_animation::EmitterAnimate;
use crate::emitters::emitter_animation::EmitterData;

pub struct SwayAnimation {
    pub from_ms: u32,
    pub until_ms: u32,
    pub start_elevation_radians: f32,
    pub end_elevation_radians: f32,
    pub start_bearing_radians: f32,
    pub end_bearing_radians: f32,
}

impl EmitterAnimate for SwayAnimation {
    fn animate(&mut self, data: &mut EmitterData, cycle_ms: u32) {
        if cycle_ms < self.from_ms || self.until_ms <= cycle_ms {
            return;
        }

        let emit_angles = &mut data.emit_options.angle_radians;
        let delta_current = cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;

        // calculate percent
        let fraction = delta_current as f32 / delta_max as f32;
        emit_angles.elevation = self.start_elevation_radians
            + fraction * (self.end_elevation_radians - self.start_elevation_radians);
        emit_angles.bearing = self.start_bearing_radians
            + fraction * (self.end_bearing_radians - self.start_bearing_radians);
    }
}
