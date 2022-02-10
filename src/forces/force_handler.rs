use bevy::prelude::Component;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use super::force::Force;
use super::force::ForceData;
use std::time::Duration;

#[derive(Component, Clone)]
pub struct ForceHandler {
    pub duration_ms: u128,
    pub lifetime: Instant,
    pub forces: Arc<Mutex<Vec<Box<dyn Force + Sync + Send>>>>,
}

impl ForceHandler {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration_ms: duration.as_millis(),
            forces: Arc::new(Mutex::new(Vec::new())),
            lifetime: Instant::now(),
        }
    }

    pub fn add(&mut self, force: Box<dyn Force + Sync + Send>) {
        let mut array = self.forces.lock().unwrap();
        array.push(force);
    }

    pub fn apply(&self, data: &mut ForceData, elapsed_ms: u128) {
        let forces_cycle_ms = elapsed_ms % self.duration_ms;

        let array = self.forces.lock().unwrap();
        for force in array.iter() {
            force.apply(data, forces_cycle_ms);
        }
    }
}
