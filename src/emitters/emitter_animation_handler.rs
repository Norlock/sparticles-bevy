use bevy::prelude::Component;

use crate::emitters::emitter_animation::{EmitterAnimate, EmitterData};

#[derive(Component)]
pub struct EmitterAnimationHandler {
    animations: Vec<Box<dyn EmitterAnimate + Sync + Send>>,
    duration_ms: u32,
}

impl EmitterAnimationHandler {
    pub fn new(duration_ms: u32, animations: Vec<Box<dyn EmitterAnimate + Sync + Send>>) -> Self {
        Self {
            animations,
            duration_ms,
        }
    }

    pub fn animate(&mut self, data: &mut EmitterData, elapsed_ms: u128) {
        let cycle_ms = elapsed_ms as u32 % self.duration_ms;
        for animation in self.animations.iter_mut() {
            animation.animate(data, cycle_ms);
        }
    }
}
