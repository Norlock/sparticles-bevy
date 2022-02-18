#[derive(Debug, Clone, Copy)]
pub struct Angles(pub f32, pub f32);

impl Angles {
    pub fn to_radians(&self) -> Angles {
        Angles(self.0.to_radians(), self.1.to_radians())
    }
}
