use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn update(&mut self, vx: f32, vy: f32, vz: f32) {
        self.x += vx;
        self.y += vy;
        self.z += vz;
    }
}
