use super::animation::Animate;
use super::animation::AnimationData;
use super::animation::AnimationTime;

pub struct SizeAnimation {
    pub start_scale: f32,
    pub end_scale: f32,
    pub from_ms: u32,
    pub until_ms: u32,
}

impl Animate for SizeAnimation {
    fn animate(&self, data: &mut AnimationData, time: &AnimationTime) {
        if time.cycle_ms < self.from_ms || self.until_ms <= time.cycle_ms {
            return;
        }

        let delta_current = time.cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;

        // calculate percent
        let fraction = delta_current as f32 / delta_max as f32;
        let scale = self.start_scale + fraction * (self.end_scale - self.start_scale);
        data.scale.x = scale;
        data.scale.y = scale;
        data.scale.z = scale;
    }
}
