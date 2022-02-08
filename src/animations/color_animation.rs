use macroquad::prelude::Color;

use super::animation::Animate;
use super::animation::AnimationData;
use super::animation::AnimationTime;

#[derive(Clone, Debug)]
pub struct DuoColorAnimation {
    pub color_from: Color,
    pub color_to: Color,
    pub from_ms: u32,
    pub until_ms: u32,
}

#[derive(Clone, Debug)]
pub struct MonoColorAnimation {
    pub color: Color,
    pub from_ms: u32,
    pub until_ms: u32,
}

impl Animate for DuoColorAnimation {
    fn animate(&self, data: &mut AnimationData, time: &AnimationTime) {
        if time.cycle_ms < self.from_ms || self.until_ms <= time.cycle_ms {
            return;
        }

        let delta_current = time.cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;

        // calculate percent from 0..1
        let fraction = delta_current as f32 / delta_max as f32;
        data.color.r = self.color_from.r + fraction * (self.color_to.r - self.color_from.r);
        data.color.g = self.color_from.g + fraction * (self.color_to.g - self.color_from.g);
        data.color.b = self.color_from.b + fraction * (self.color_to.b - self.color_from.b);
        data.color.a = self.color_from.a + fraction * (self.color_to.a - self.color_from.a);
    }
}

impl Animate for MonoColorAnimation {
    fn animate(&self, data: &mut AnimationData, time: &AnimationTime) {
        if time.cycle_ms < self.from_ms || self.until_ms <= time.cycle_ms {
            return;
        }

        let delta_current = time.cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;
        let fraction = delta_current as f32 / delta_max as f32;

        let fraction_remainder = 1. - fraction;
        let delta_r = (self.color.r - data.color.r) / fraction_remainder;
        let delta_g = (self.color.g - data.color.g) / fraction_remainder;
        let delta_b = (self.color.b - data.color.b) / fraction_remainder;
        let delta_a = (self.color.a - data.color.a) / fraction_remainder;

        let from_r = (self.color.r - delta_r).abs();
        let from_g = (self.color.g - delta_g).abs();
        let from_b = (self.color.b - delta_b).abs();
        let from_a = (self.color.a - delta_a).abs();

        data.color.r = from_r + fraction * (self.color.r - from_r); // - data.color.r);
        data.color.g = from_g + fraction * (self.color.g - from_g); // - data.color.g);
        data.color.b = from_b + fraction * (self.color.b - from_b); // - data.color.b);
        data.color.a = from_a + fraction * (self.color.a - from_a); // - data.color.a);
    }
}
