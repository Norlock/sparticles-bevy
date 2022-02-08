#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            width: 0.,
            height: 0.,
        }
    }
}
