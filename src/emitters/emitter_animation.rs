use bevy::prelude::Transform;

use std::fmt::Debug;

use super::emitter::{EmitOptions, EmitterParticleAttributes, Velocity};

pub trait EmitterAnimate {
    fn animate(&mut self, data: &mut EmitterData, cycle_ms: u32);
}

impl Debug for dyn EmitterAnimate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Animate")
    }
}

pub struct EmitterData<'a> {
    pub particle_attributes: &'a mut EmitterParticleAttributes,
    pub emit_options: &'a mut EmitOptions,
    pub transform: &'a mut Transform,
    pub velocity: &'a mut Velocity,
    pub delta_seconds: f32,
}
