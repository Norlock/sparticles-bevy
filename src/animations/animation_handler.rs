use bevy::prelude::Component;
use rand::{thread_rng, Rng};

use super::animation::{Animate, AnimationData, AnimationTime};
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Component)]
pub struct AnimationHandler {
    animation_offset_ms: u32,
    animations: Vec<Box<dyn Animate + Sync + Send>>,
    duration_ms: u32,
}

pub enum StartAnimationAt {
    Zero,
    Random,
    RangeMs(u32, u32),
}

pub struct AnimationOptions {
    pub animations: Vec<Box<dyn Animate + Sync + Send>>,
    pub duration_ms: u32,
    pub start_at: StartAnimationAt,
}

impl Debug for StartAnimationAt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "StartAnimationAt")
    }
}

impl AnimationHandler {
    pub fn new(options: AnimationOptions) -> Self {
        let mut rng = thread_rng();
        let animation_offset_ms = match options.start_at {
            StartAnimationAt::Zero => 0,
            StartAnimationAt::Random => rng.gen_range(0..options.duration_ms),
            StartAnimationAt::RangeMs(start, end) => rng.gen_range(start..end),
        };

        AnimationHandler {
            animation_offset_ms,
            animations: options.animations,
            duration_ms: options.duration_ms,
        }
    }

    pub fn apply(&mut self, data: &mut AnimationData, elapsed_ms: u128) {
        let cycle_ms = (elapsed_ms as u32 + self.animation_offset_ms) % self.duration_ms;

        let time = AnimationTime {
            cycle_ms,
            total_ms: elapsed_ms,
        };

        for animation in self.animations.iter() {
            animation.animate(data, &time);
        }
    }
}

impl AnimationOptions {
    pub fn new(
        duration_ms: u32,
        start_at: StartAnimationAt,
        animations: Vec<Box<dyn Animate + Send + Sync>>,
    ) -> Self {
        Self {
            duration_ms,
            start_at,
            animations,
        }
    }
}
