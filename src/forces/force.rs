use std::fmt::Debug;

use bevy::math::Vec3;

use crate::emitters::emitter::Velocity;

pub trait Force {
    fn apply(&self, particle: &mut ForceData, force_cycle_ms: u128);
}

impl Debug for dyn Force {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Force")
    }
}

pub struct ForceData<'a, 'b> {
    pub position: &'a Vec3,
    pub velocity: &'b mut Velocity,
    pub radius: Vec3,
    pub mass: f32,
    pub delta_seconds: f32,
}
