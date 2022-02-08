use crate::animations::animation::AnimationData;
use crate::animations::animation_handler::AnimationHandler;
use crate::animations::animation_handler::AnimationOptions;
use crate::collision::CollisionData;
use crate::trails::trail_animation::TrailData;
use crate::trails::trail_handler::TrailHandler;
use macroquad::prelude::*;
use std::rc::Rc;
use std::time::Instant;

use crate::position::Position;

#[derive(Debug)]
pub struct Particle {
    pub queue_frame: u64,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub diameter: f32,
    pub color: Color,
    texture: Option<Texture2D>,
    pub mass: f32,
    /// number between 0 and 1.
    pub elasticity: f32,
    /// number between 0 and 1. E.g. 0.008
    pub friction_coefficient: f32,
    pub lifetime: Rc<Instant>,
    pub animation_handler: Option<AnimationHandler>,
    pub trail_handler: Option<TrailHandler>,
}

pub struct ParticleAttributes {
    /// number between 0 and 1. (percentage of bounciness).
    pub elasticity: f32,
    /// number between 0 and 1. E.g. 0.008
    pub friction_coefficient: f32,

    pub color: Color,
    pub texture: Option<Texture2D>,
    pub mass: f32,
    pub diameter: f32,
    pub animation_options: Option<AnimationOptions>,
    pub trail_handler: Option<TrailHandler>,
}

impl Particle {
    pub fn new(x: f32, y: f32, attributes: &ParticleAttributes, lifetime: Rc<Instant>) -> Self {
        let animation_handler = AnimationHandler::new(&attributes.animation_options);

        Self {
            x,
            y,
            vx: 0.,
            vy: 0.,
            friction_coefficient: attributes.friction_coefficient,
            color: attributes.color,
            texture: attributes.texture,
            radius: attributes.diameter / 2.,
            diameter: attributes.diameter,
            elasticity: attributes.elasticity,
            mass: attributes.mass,
            queue_frame: u64::MAX,
            lifetime,
            trail_handler: attributes.trail_handler.clone(),
            animation_handler,
        }
    }

    fn move_if_overlaps(&mut self, other: &mut Particle) {
        let new_x = self.x + self.vx;
        let new_y = self.y + self.vy;
        let other_new_x = other.x + other.vx;
        let other_new_y = other.y + other.vy;
        let end_other_new_x = other_new_x + other.diameter;
        let end_other_new_y = other_new_y + other.diameter;

        let x_difference = new_x - other_new_x;
        let y_difference = new_y - other_new_y;

        let collision_self_placed_right = 0. < x_difference && x_difference < other.diameter;
        let collision_self_placed_left = -self.diameter < x_difference && x_difference < 0.;
        let collision_self_placed_bottom = 0. < y_difference && y_difference < other.diameter;
        let collision_self_placed_top = -self.diameter < y_difference && y_difference < 0.;

        let x_collision = collision_self_placed_left || collision_self_placed_right;
        let y_collision = collision_self_placed_top || collision_self_placed_bottom;

        if !x_collision || !y_collision {
            return;
        }

        // The higher the difference the lower the overlap is.
        // In case of overlap, you move back the least overlapping part.
        // e.g. both particles diameter is 5.
        // p1,x == 9, p2,x == 5. Remainder == 9 - 5. (4)
        // p1,y == 8, p2,y == 5. Remainder == 8 - 5. (3)
        // if x1 moves to the right with 1x its out of bound instead of y1 moving with 2y.
        let move_back_horizontally = y_difference.abs() < x_difference.abs();

        if move_back_horizontally {
            if collision_self_placed_right {
                if 0. <= self.vx {
                    self.x = end_other_new_x - self.vx + 0.01;
                } else {
                    self.x = end_other_new_x + self.vx + 0.01;
                }
            } else if 0. <= other.vx {
                self.x = other_new_x - self.diameter - other.vx - 0.01;
            } else {
                self.x = other_new_x - self.diameter + other.vx - 0.01;
            }
        } else {
            if collision_self_placed_bottom {
                if 0. <= self.vy {
                    self.y = end_other_new_y - self.vy + 0.01;
                } else {
                    self.y = end_other_new_y + self.vy + 0.01;
                }
            } else if 0. <= other.vy {
                self.y = other_new_y - self.diameter - other.vy - 0.01;
            } else {
                self.y = other_new_y - self.diameter + other.vy - 0.01;
            }
        }
    }

    pub fn handle_possible_collision(
        &mut self,
        other: &mut Particle,
        data: &mut CollisionData,
    ) -> bool {
        let CollisionData {
            new_x,
            new_y,
            end_new_x,
            end_new_y,
        } = *data;

        let other_new_x = other.x + other.vx;
        let other_new_y = other.y + other.vy;
        let end_other_new_x = other_new_x + other.diameter;
        let end_other_new_y = other_new_y + other.diameter;

        let inside_x = other_new_x <= new_x && new_x <= end_other_new_x
            || other_new_x <= end_new_x && end_new_x <= end_other_new_x;
        let inside_y = other_new_y <= new_y && new_y <= end_other_new_y
            || other_new_y <= end_new_y && end_new_y <= end_other_new_y;

        if !inside_x || !inside_y {
            return false; // No collision
        }

        if self.mass == other.mass {
            let tmp = self.vx;
            self.vx = other.vx * self.elasticity;
            other.vx = tmp * other.elasticity;

            let tmp = self.vy;
            self.vy = other.vy * self.elasticity;
            other.vy = tmp * other.elasticity;
            self.move_if_overlaps(other);
            return true;
        }

        let total_weight = self.mass + other.mass;

        let transform_vx = ((self.mass - other.mass) / total_weight * self.vx)
            + (2. * other.mass / total_weight * other.vx);
        let other_vx = (2. * self.mass / total_weight * self.vx)
            + ((other.mass - self.mass) / total_weight * other.vx);

        self.vx = transform_vx * self.elasticity;
        other.vx = other_vx * other.elasticity;

        let transform_vy = ((self.mass - other.mass) / total_weight * self.vy)
            + (2. * other.mass / total_weight * other.vy);
        let other_vy = (2. * self.mass / total_weight * self.vy)
            + ((other.mass - self.mass) / total_weight * other.vy);

        self.vy = transform_vy * self.elasticity;
        other.vx = other_vy * other.elasticity;

        self.move_if_overlaps(other);
        true
    }

    pub fn animate(&mut self) {
        if let Some(animator) = &mut self.animation_handler {
            let mut data = AnimationData {
                color: self.color,
                radius: self.radius,
                vx: self.vx,
                vy: self.vy,
            };

            let elapsed_ms = self.lifetime.elapsed().as_millis();
            animator.animate(&mut data, elapsed_ms);

            self.color = data.color;
            self.radius = data.radius;
            self.diameter = data.radius * 2.;
        }
    }

    pub fn draw(&mut self, grid_position: &Position) {
        let x = self.x + grid_position.x;
        let y = self.y + grid_position.y;

        if let Some(trail_handler) = &mut self.trail_handler {
            let elapsed_ms = self.lifetime.elapsed().as_millis();

            let data = TrailData {
                radius: self.radius,
                color: self.color,
                x_abs: x,
                y_abs: y,
            };

            trail_handler.animate(&data, elapsed_ms);
        }

        if let Some(texture) = self.texture {
            let side = self.diameter;
            let dest_size = Some(Vec2::new(side, side));

            let params = DrawTextureParams {
                dest_size,
                ..Default::default()
            };

            draw_texture_ex(texture, x, y, self.color, params);
        } else {
            draw_circle(x, y, self.radius, self.color);
        }
    }

    pub fn apply_friction(&mut self) {
        let x_loss = self.vx * self.mass * self.friction_coefficient;
        let y_loss = self.vy * self.mass * self.friction_coefficient;

        self.vx -= x_loss / self.mass;
        self.vy -= y_loss / self.mass;
    }

    pub fn transform(&mut self, max_width: f32, max_height: f32) {
        self.x += self.vx;
        self.y += self.vy;

        if self.x < 0. {
            self.x = 0.;
        } else if max_width <= self.x + self.diameter {
            self.x = max_width - 0.1 - self.diameter;
        }

        if self.y < 0. {
            self.y = 0.;
        } else if max_height <= self.y + self.diameter {
            self.y = max_height - 0.1 - self.diameter;
        }
    }
}
