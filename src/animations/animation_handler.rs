use super::animation::{Animate, AnimationData, AnimationTime};
use macroquad::prelude::rand;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct AnimationHandler {
    animation_offset_ms: u32,
    iteration: u32,
    animations: Rc<Vec<Box<dyn Animate>>>,
    duration_ms: u32,
}

pub enum StartAnimationAt {
    Zero,
    Random,
    RangeMs(u32, u32),
}

#[derive(Debug)]
pub struct AnimationOptions {
    pub animations: Rc<Vec<Box<dyn Animate>>>,
    pub duration_ms: u32,
    pub start_at: StartAnimationAt,
}

impl Debug for StartAnimationAt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "StartAnimationAt")
    }
}

impl AnimationHandler {
    pub fn new(options: &Option<AnimationOptions>) -> Option<Self> {
        match options {
            Some(ah) => {
                let animation_offset_ms = match ah.start_at {
                    StartAnimationAt::Zero => 0,
                    StartAnimationAt::Random => rand::gen_range(0, ah.duration_ms),
                    StartAnimationAt::RangeMs(start, end) => rand::gen_range(start, end),
                };
                Some(AnimationHandler {
                    iteration: 0,
                    animation_offset_ms,
                    animations: Rc::clone(&ah.animations),
                    duration_ms: ah.duration_ms,
                })
            }
            None => None,
        }
    }

    pub fn animate(&mut self, data: &mut AnimationData, elapsed_ms: u128) {
        //let new_iteration = if elapsed_ms == 0 {
        //0
        //} else {
        //self.duration_ms / elapsed_ms
        //};

        //if self.iteration < new_iteration {
        //self.iteration = new_iteration;
        //}

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
        animations: Vec<Box<dyn Animate>>,
    ) -> Self {
        Self {
            duration_ms,
            start_at,
            animations: Rc::new(animations),
        }
    }
}
