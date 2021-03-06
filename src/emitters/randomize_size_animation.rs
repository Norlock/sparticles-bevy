use super::emitter_animation::EmitterAnimate;
use crate::emitters::emitter_animation::EmitterData;
use rand::{thread_rng, Rng};

pub struct RandomizeSizeAnimation {
    pub min_radius: f32,
    pub max_radius: f32,
}

// TODO over nadenken radius wordt niet gebruikt.
impl EmitterAnimate for RandomizeSizeAnimation {
    fn animate(&mut self, data: &mut EmitterData, _: u32) {
        data.particle_attributes.radius = thread_rng().gen_range(self.min_radius..self.max_radius);
    }
}
