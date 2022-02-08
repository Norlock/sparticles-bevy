use crate::point::Point;

// Moves emitter over the flight path.
#[derive(Debug)]
pub struct MovementHandler {
    flight_path: Vec<Point>,
    /// Applied each frame
    speed: f32,
    vx: f32,
    vy: f32,
    towards_point_index: usize,
}

impl MovementHandler {
    pub fn new(speed: f32) -> Self {
        Self {
            flight_path: Vec::new(),
            towards_point_index: 0,
            vx: 0.,
            vy: 0.,
            speed,
        }
    }

    pub fn travel(&mut self, current_point: Point) -> Point {
        let to_point = self.flight_path[self.towards_point_index];
        let new_x = current_point.0 + self.vx;
        let new_y = current_point.1 + self.vy;

        let x_before = (to_point.0 - current_point.0).is_sign_positive();
        let y_before = (to_point.1 - current_point.1).is_sign_positive();
        let x_after = (to_point.0 - new_x).is_sign_positive();
        let y_after = (to_point.1 - new_y).is_sign_positive();

        if new_x == to_point.0 && new_y == to_point.1 {
            self.change_direction(current_point)
        } else if x_before == x_after && y_before == y_after {
            Point(new_x, new_y)
        } else {
            self.vx = to_point.0 - new_x;
            self.vy = to_point.1 - new_y;
            to_point
        }
    }

    pub fn change_direction(&mut self, current_point: Point) -> Point {
        if self.towards_point_index + 1 < self.flight_path.len() {
            self.towards_point_index += 1;
        } else {
            self.towards_point_index = 0;
        }

        // Update vx, vy
        let new_point = &mut self.flight_path[self.towards_point_index];

        let x_abs = current_point.0 - new_point.0;
        let y_abs = current_point.1 - new_point.1;

        let angle = y_abs / x_abs;

        self.vx = self.speed * angle.cos();
        self.vy = self.speed * angle.sin();

        Point(current_point.0 + self.vx, current_point.1 + self.vy)
    }
}
