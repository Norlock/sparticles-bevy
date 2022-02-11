use bevy::prelude::Component;
use std::time::Instant;

use super::force::Force;
use super::force::ForceData;
use std::time::Duration;

#[derive(Component)]
pub struct ForceHandler {
    pub duration_ms: u128,
    pub lifetime: Instant,
    pub forces: Vec<Box<dyn Force + Sync + Send>>,
}

impl ForceHandler {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration_ms: duration.as_millis(),
            forces: Vec::new(),
            lifetime: Instant::now(),
        }
    }

    pub fn add(&mut self, force: Box<dyn Force + Sync + Send>) {
        self.forces.push(force);
    }

    pub fn apply(&self, data: &mut ForceData, elapsed_ms: u128) {
        let forces_cycle_ms = elapsed_ms % self.duration_ms;

        for force in self.forces.iter() {
            force.apply(data, forces_cycle_ms);
        }
    }
}
