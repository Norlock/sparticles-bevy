use super::animation::Animate;
use super::animation::AnimationData;
use super::animation::AnimationTime;

pub struct WeirdAnimation {
    from_ms: u32,
    until_ms: u32,
    strayness_radians: f32,
}

impl WeirdAnimation {
    pub fn new(from_ms: u32, until_ms: u32, strayness_degrees: f32) -> Self {
        Self {
            from_ms,
            until_ms,
            strayness_radians: strayness_degrees.to_radians(),
        }
    }
}

impl Animate for WeirdAnimation {
    fn animate(&self, data: &mut AnimationData, time: &AnimationTime) {
        if time.cycle_ms < self.from_ms || self.until_ms <= time.cycle_ms {
            return;
        }

        let velocity = &mut data.velocity;

        let vortex_speed = 10.;
        let vortex_scale = 0.5;
        let vx = -velocity.vy * vortex_speed + velocity.vx;
        let vy = velocity.vx * vortex_speed + velocity.vy;
        let vz = -velocity.vz * vortex_speed;

        let mut factor = 1.
            / (1.
                + (velocity.vx.powi(2) + velocity.vy.powi(2) + velocity.vz.powi(2)) / vortex_scale);

        let life_factor = 0.1;
        factor *= (1. - life_factor) * life_factor * 4.;
        velocity.vx += (vx - velocity.vx) * factor;
        velocity.vy += (vy - velocity.vy) * factor;
        velocity.vz += (vz - velocity.vz) * factor;
    }
}
