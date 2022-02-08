use macroquad::prelude::Color;
use std::fmt::Debug;

pub const FRAME_TIME: u32 = 16;

pub struct AnimationTime {
    pub cycle_ms: u32,
    pub total_ms: u128,
}

pub trait Animate {
    fn animate(&self, data: &mut AnimationData, time: &AnimationTime);
}

impl Debug for dyn Animate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Animate")
    }
}

pub struct AnimationData {
    pub color: Color,
    pub radius: f32,
    pub vx: f32,
    pub vy: f32,
}
