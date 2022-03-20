use crate::emitters::emitter::Velocity;
use bevy::math::Vec3;
use bevy::render::color::Color;
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

pub struct AnimationData<'a, 'b, 'c> {
    pub color: &'a mut [f32; 4],
    pub scale: &'b mut Vec3,
    pub velocity: &'c mut Velocity,
}
