use super::emitter_animation::EmitterData;
use bevy::prelude::Color;

use super::emitter_animation::EmitterAnimate;

pub struct EmitColorAnimation {
    pub from_ms: u32,
    pub until_ms: u32,
    pub from_color: Color,
    pub to_color: Color,
}

impl EmitterAnimate for EmitColorAnimation {
    fn animate(&mut self, data: &mut EmitterData, cycle_ms: u32) {
        if cycle_ms < self.from_ms || self.until_ms <= cycle_ms {
            return;
        }

        let delta_current = cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;

        let color = &mut data.particle_attributes.color;

        // calculate percent from 0..1
        let fraction = delta_current as f32 / delta_max as f32;
        let r = self.from_color.r() + fraction * (self.to_color.r() - self.from_color.r());
        let g = self.from_color.g() + fraction * (self.to_color.g() - self.from_color.g());
        let b = self.from_color.b() + fraction * (self.to_color.b() - self.from_color.b());
        let a = self.from_color.a() + fraction * (self.to_color.a() - self.from_color.a());

        color.set_r(r);
        color.set_g(g);
        color.set_b(b);
        color.set_a(a);
    }
}
