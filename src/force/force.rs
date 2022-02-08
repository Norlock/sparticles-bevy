use std::fmt::Debug;

pub trait Force {
    fn apply(&self, particle: &mut ForceData, force_cycle_ms: u128);
}

impl Debug for dyn Force {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Force")
    }
}

pub struct ForceData {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub mass: f32,
}
