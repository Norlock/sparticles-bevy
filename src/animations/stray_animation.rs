use rand::thread_rng;
use rand::Rng;

use super::animation::Animate;
use super::animation::AnimationData;
use super::animation::AnimationTime;

pub struct StrayAnimation {
    from_ms: u32,
    until_ms: u32,
    strayness_radians: f32,
}

impl StrayAnimation {
    pub fn new(from_ms: u32, until_ms: u32, strayness_degrees: f32) -> Self {
        Self {
            from_ms,
            until_ms,
            strayness_radians: strayness_degrees.to_radians(),
        }
    }
}

// TODO nog een keer naar kijken.
impl Animate for StrayAnimation {
    fn animate(&self, data: &mut AnimationData, time: &AnimationTime) {
        if time.cycle_ms < self.from_ms || self.until_ms <= time.cycle_ms {
            return;
        }

        let mut rng = thread_rng();
        //let elevation = rng.gen_range(-self.strayness_radians..self.strayness_radians);
        let elevation = rng.gen_range(-self.strayness_radians..self.strayness_radians);
        let bearing = rng.gen_range(-self.strayness_radians..self.strayness_radians);
        let velocity = &mut data.velocity;

        let stray_vx = velocity.vx * elevation.cos() - velocity.vy * elevation.sin();
        let stray_vy = velocity.vy * elevation.cos() + velocity.vx * elevation.sin();

        velocity.vx = stray_vx;
        velocity.vy = stray_vy;

        //let speed = velocity.vy.powi(2) + velocity.vz.powi(2) + velocity.vx.powi(2);
        //println!(
        //"vx oud {} vy oud {} vz oud {}",
        //velocity.vx, velocity.vy, velocity.vz
        //);
        //println!("sb {}", speed.sqrt());

        let bearing_sin = bearing.sin();
        let bearing_cos = bearing.cos();

        let bearing_vx = velocity.vx * bearing_cos + velocity.vz * bearing_sin
            - velocity.vy * bearing_sin
            - velocity.vz * bearing_sin;
        let bearing_vy = velocity.vy * bearing_cos
            + velocity.vz * bearing_sin
            + velocity.vx * bearing_sin
            + velocity.vz * bearing_sin;
        let bearing_vz =
            velocity.vz * bearing_cos - velocity.vy * bearing_sin - velocity.vx * bearing_sin;

        velocity.vy = bearing_vy;
        velocity.vz = bearing_vz;
        velocity.vx = bearing_vx;

        //let speed = (bearing_vy.powi(2) + bearing_vz.powi(2) + bearing_vx.powi(2)).sqrt();
        //println!(
        //"vx new {} vy new {} vz new {}",
        //velocity.vx, velocity.vy, velocity.vz
        //);
        //println!("sa {}", speed);
    }
}
