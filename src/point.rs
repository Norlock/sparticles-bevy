#[derive(Debug, Clone, Copy)]
pub struct Point(pub f32, pub f32);

pub type Degrees = Point;
pub type Radians = Point;

impl Point {
    pub fn to_radians(&self) -> Point {
        Point(self.0.to_radians(), self.1.to_radians())
    }
}
