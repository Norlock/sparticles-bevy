use crate::force::force::Force;
use crate::force::force::ForceData;
use std::time::Duration;

#[derive(Debug)]
pub struct ForceHandler {
    pub iteration: u16,
    pub duration_ms: u128,
    pub forces: Vec<Box<dyn Force>>,
}

impl ForceHandler {
    pub fn new(duration: Duration) -> Self {
        Self {
            iteration: 0,
            duration_ms: duration.as_millis(),
            forces: Vec::new(),
        }
    }

    pub fn add(&mut self, force: Box<dyn Force>) {
        self.forces.push(force);
    }

    pub fn apply(&self, particle: &mut ForceData, elapsed_ms: u128) {
        let forces_cycle_ms = elapsed_ms % self.duration_ms;

        for force in self.forces.iter() {
            force.apply(particle, forces_cycle_ms);
        }
    }
}
