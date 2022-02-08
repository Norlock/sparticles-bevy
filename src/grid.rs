use crate::emitters::emitter::Emitter;
use crate::emitters::emitter::EmitterOptions;
use crate::force::force::ForceData;
use crate::force::force_handler::ForceHandler;
use std::{rc::Rc, time::Instant};

use crate::{
    collision::CollisionData,
    fill_style::FillStyle,
    particle::{Particle, ParticleAttributes},
    position::Position,
};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Grid {
    pub possibility_spots: Vec<Vec<Particle>>,
    pub cell_x_count: usize,
    pub cell_y_count: usize,
    pub possibility_x_count: usize,
    pub possibility_y_count: usize,
    pub possibility_side_length: usize,
    pub position: Position,
    pub frame: u64,
    pub cell_width: usize,
    pub cell_height: usize,
    pub duration: u128,
    pub fps: i32,
    pub particle_count: u32,
    pub emitted_particle_count: u32,
    pub force_handler: Option<ForceHandler>,
    pub emitters: Vec<Emitter>,
    pub lifetime: Instant,
}

pub struct GridOptions {
    pub cell_x_count: usize,
    pub cell_y_count: usize,
    pub possibility_x_count: usize,
    pub possibility_y_count: usize,
    pub possibility_side_length: usize,
    pub position: Position,
    pub force_handler: Option<ForceHandler>,
}

fn create_possibility_grid(
    possibility_x_count: usize,
    possiblity_y_count: usize,
) -> Vec<Vec<Particle>> {
    let mut spots: Vec<Vec<Particle>> = Vec::new();

    for _ in 0..possibility_x_count {
        for _ in 0..possiblity_y_count {
            spots.push(Vec::new());
        }
    }

    spots
}

impl Grid {
    pub fn new(options: GridOptions) -> Self {
        let GridOptions {
            cell_x_count,
            cell_y_count,
            possibility_x_count,
            possibility_y_count,
            possibility_side_length,
            mut position,
            force_handler,
        } = options;
        let cell_width = possibility_x_count * possibility_side_length;
        let cell_height = possibility_y_count * possibility_side_length;
        position.width = (cell_x_count * cell_width) as f32;
        position.height = (cell_y_count * cell_height) as f32;

        let possibility_spots = create_possibility_grid(possibility_x_count, possibility_y_count);

        Self {
            cell_x_count,
            cell_y_count,
            possibility_x_count,
            possibility_y_count,
            possibility_side_length,
            position,
            possibility_spots,
            cell_width,
            cell_height,
            frame: 0,
            duration: 0,
            fps: 0,
            particle_count: 0,
            emitted_particle_count: 0,
            force_handler,
            emitters: Vec::new(),
            lifetime: Instant::now(),
        }
    }

    pub fn debug(&self) {
        for v_index in 0..self.possibility_spots.len() {
            println!(
                "possiblity x: {}, y: {}, has {} particles.",
                v_index % self.possibility_x_count,
                v_index / self.possibility_x_count,
                self.possibility_spots[v_index].len()
            );
        }
        println!("------------------");
    }

    fn cell_x_index(&self, x_coord: f32) -> usize {
        x_coord as usize / self.cell_width
    }

    fn cell_y_index(&self, y_coord: f32) -> usize {
        y_coord as usize / self.cell_height
    }

    fn possibility_x_index(&self, x_coord: f32) -> usize {
        let x_residual = x_coord as usize % self.cell_width;
        x_residual / self.possibility_side_length
    }

    fn possibility_y_index(&self, y_coord: f32) -> usize {
        let y_residual = y_coord as usize % self.cell_height;
        y_residual / self.possibility_side_length
    }

    fn possibility_index(&self, x_index: usize, y_index: usize) -> usize {
        self.possibility_x_count * y_index + x_index
    }

    fn vec_spot_particle(&self, particle: &Particle) -> usize {
        let new_x_spot = self.possibility_x_index(particle.x);
        let new_y_spot = self.possibility_y_index(particle.y);

        self.possibility_index(new_x_spot, new_y_spot)
    }

    fn has_collision(
        &mut self,
        particle: &mut Particle,
        data: &mut CollisionData,
        vec_index: usize,
    ) -> bool {
        for other in self.possibility_spots[vec_index].iter_mut() {
            if particle.handle_possible_collision(other, data) {
                return true;
            }
        }
        false
    }

    fn handle_collision(&mut self, particle: &mut Particle, data: &mut CollisionData) -> usize {
        let CollisionData {
            new_x,
            new_y,
            end_new_x,
            end_new_y,
        } = *data;

        let new_x_spot = self.possibility_x_index(new_x);
        let new_y_spot = self.possibility_y_index(new_y);

        let new_vec_index = self.possibility_index(new_x_spot, new_y_spot);

        if self.has_collision(particle, data, new_vec_index) {
            return self.vec_spot_particle(particle);
        }

        let end_x_spot = self.possibility_x_index(end_new_x);
        let end_y_spot = self.possibility_y_index(end_new_y);

        let has_diff_end_x_spot = end_x_spot != new_x_spot;
        let has_diff_end_y_spot = end_y_spot != new_y_spot;

        if has_diff_end_x_spot {
            let new_vec_index = self.possibility_index(end_x_spot, new_y_spot);

            if self.has_collision(particle, data, new_vec_index) {
                return self.vec_spot_particle(particle);
            }
        }

        if has_diff_end_y_spot {
            let new_vec_index = self.possibility_index(new_x_spot, end_y_spot);

            if self.has_collision(particle, data, new_vec_index) {
                return self.vec_spot_particle(particle);
            }
        }

        if has_diff_end_x_spot && has_diff_end_y_spot {
            let new_vec_index = self.possibility_index(end_x_spot, end_y_spot);

            if self.has_collision(particle, data, new_vec_index) {
                return self.vec_spot_particle(particle);
            }
        }

        new_vec_index
    }

    fn update_spot(&mut self, vec_index: usize, spot_index: usize, elapsed_ms: u128) {
        let mut particle = self.possibility_spots[vec_index].swap_remove(spot_index);

        let mut data = ForceData {
            x: particle.x,
            y: particle.y,
            vx: particle.vx,
            vy: particle.vy,
            radius: particle.radius,
            mass: particle.mass,
        };

        if let Some(force_handler) = &mut self.force_handler {
            force_handler.apply(&mut data, elapsed_ms);
            particle.vx = data.vx;
            particle.vy = data.vy;
        }

        let new_vec_index = self.update_particle(&mut particle);

        if new_vec_index != vec_index {
            particle.queue_frame = self.frame;
        }

        self.possibility_spots[new_vec_index].push(particle);
    }

    /**
     * returns true if index needs to be incremented
     */
    fn update_particle(&mut self, particle: &mut Particle) -> usize {
        let new_x = particle.x + particle.vx;
        let new_y = particle.y + particle.vy;
        let end_new_x = new_x + particle.diameter;
        let end_new_y = new_y + particle.diameter;

        let x_out_of_bounds = new_x < 0. || self.position.width <= end_new_x;
        let y_out_of_bounds = new_y < 0. || self.position.height <= end_new_y;

        // Inverse direction.
        let elasticity_force = -1. * particle.elasticity;

        if x_out_of_bounds {
            particle.vx *= elasticity_force;
        }

        if y_out_of_bounds {
            particle.vy *= elasticity_force;
        }

        particle.apply_friction();
        particle.animate();

        let mut data = CollisionData {
            new_x,
            new_y,
            end_new_x,
            end_new_y,
        };

        let new_vec_index = self.handle_collision(particle, &mut data);

        particle.transform(self.position.width, self.position.height);
        particle.draw(&self.position);

        new_vec_index
    }

    pub fn fill(&mut self, attributes: &ParticleAttributes, count: u32, fill_style: FillStyle) {
        self.particle_count += count;

        match fill_style {
            FillStyle::WhiteNoise => self.fill_white_noise(attributes, count),
        }
    }

    pub fn add_emitter(&mut self, options: EmitterOptions) {
        self.emitters.push(Emitter::new(self.position, options));
    }

    pub fn draw(&mut self) {
        let update_gui = self.frame % 50 == 0;
        if update_gui {
            self.duration = self.lifetime.elapsed().as_micros();
        }

        let elapsed_ms = self.lifetime.elapsed().as_millis();

        for vec_index in 0..self.possibility_spots.len() {
            for spot_index in (0..self.possibility_spots[vec_index].len()).rev() {
                if self.possibility_spots[vec_index][spot_index].queue_frame == self.frame {
                    continue;
                }

                self.update_spot(vec_index, spot_index, elapsed_ms);
            }
        }

        for i in (0..self.emitters.len()).rev() {
            let mut emitter = self.emitters.swap_remove(i);
            emitter.emit();

            if !emitter.delete {
                self.emitters.push(emitter);
            }
        }

        self.frame += 1;

        if update_gui {
            let end = self.lifetime.elapsed().as_micros();
            self.duration = end - self.duration;
            self.fps = get_fps();

            self.emitted_particle_count = self
                .emitters
                .iter()
                .fold(0, |acc, emitter| acc + emitter.particle_count);
        }
    }

    pub fn draw_ui(&mut self) {
        draw_text(
            format!(
                "Particle count: {}",
                self.particle_count + self.emitted_particle_count
            )
            .as_str(),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        draw_text(
            format!("Loop time (micro): {}", self.duration).as_str(),
            10.0,
            40.0,
            20.0,
            WHITE,
        );

        draw_text(
            format!("FPS: {}", self.fps).as_str(),
            10.0,
            60.0,
            20.0,
            WHITE,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let (raw_x, raw_y) = mouse_position();
            // x, y on screen is an offset of the the grid position.
            let x = raw_x - self.position.x;
            let y = raw_y - self.position.y;
            let x_index = self.possibility_x_index(x);
            let y_index = self.possibility_y_index(y);
            let vec_index = self.possibility_index(x_index, y_index);

            println!("------------------------------");
            for particle in self.possibility_spots[vec_index].iter() {
                let inside_x = particle.x <= x && x <= particle.x + particle.diameter;
                let inside_y = particle.y <= y && y <= particle.y + particle.diameter;

                // No collision
                if inside_x && inside_y {
                    println!("{:#?}", particle);
                }
            }
        }

        fn draw_grid(grid: &Grid) {
            for x_index in 0..grid.possibility_x_count * grid.cell_x_count {
                for y_index in 0..grid.possibility_y_count * grid.cell_y_count {
                    let x = grid.position.x + (x_index * grid.possibility_side_length) as f32;
                    let y = grid.position.y + (y_index * grid.possibility_side_length) as f32;

                    draw_rectangle_lines(
                        x,
                        y,
                        grid.possibility_side_length as f32,
                        grid.possibility_side_length as f32,
                        0.3,
                        LIGHTGRAY,
                    );
                }
            }
        }

        //draw_grid(&self);
    }

    fn possibility_taken(&self, x_coord: f32, y_coord: f32) -> bool {
        let cell_x_index = self.cell_x_index(x_coord);
        let cell_y_index = self.cell_y_index(y_coord);

        let poss_x_index = self.possibility_x_index(x_coord);
        let poss_y_index = self.possibility_y_index(y_coord);

        self.possibility_spots[self.possibility_index(poss_x_index, poss_y_index)]
            .iter()
            .any(|p| {
                self.cell_x_index(p.x) == cell_x_index && self.cell_y_index(p.y) == cell_y_index
            })
    }

    fn fill_white_noise(&mut self, attributes: &ParticleAttributes, count: u32) {
        let mut i: u32 = 0;
        let lifetime = Rc::new(Instant::now());
        while i < count {
            let x_coord = rand::gen_range(0., self.position.width);
            let y_coord = rand::gen_range(0., self.position.height);
            if !self.possibility_taken(x_coord, y_coord) {
                self.add_particle(x_coord, y_coord, attributes, lifetime.clone());
                i += 1;
            }
        }
    }

    fn add_particle(
        &mut self,
        x_coord: f32,
        y_coord: f32,
        attributes: &ParticleAttributes,
        lifetime: Rc<Instant>,
    ) {
        let particle = Particle::new(x_coord, y_coord, attributes, lifetime);
        let poss_x_index = self.possibility_x_index(x_coord);
        let poss_y_index = self.possibility_y_index(y_coord);
        let poss_index = self.possibility_index(poss_x_index, poss_y_index);
        self.possibility_spots[poss_index].push(particle);
    }
}

#[cfg(test)]
mod test {
    use crate::force::force;
    use crate::grid::test::force::Force;
    use crate::particle::*;
    use crate::FillStyle;
    use crate::ForceBuilder;
    use crate::ForceType;
    use crate::Grid;
    use crate::GridOptions;
    use crate::ParticleAttributes;
    use crate::Position;
    use macroquad::prelude::Color;

    fn default_grid() -> Grid {
        let options = GridOptions {
            cell_x_count: 5,
            cell_y_count: 5,
            possibility_x_count: 10,
            possibility_y_count: 10,
            possibility_side_length: 10,
            position: Position::new(1., 2.),
            force_handler: Vec::new(),
        };

        Grid::new(options)
    }

    fn default_attributes() -> ParticleAttributes {
        ParticleAttributes {
            color: Color::from_rgba(20, 20, 200, 255),
            friction_coefficient: 0.5,
            diameter: 5.,
            elasticity: 0.9,
            mass: 1.,
            animation_start_at: InitAnimation::Zero,
        }
    }

    fn default_forces() -> Vec<Force> {
        let mut builder = ForceBuilder::new();

        builder.add(
            ForceType::Static {
                vx: 0.02,
                vy: 0.015,
            },
            50,
        );

        builder.add(ForceType::None, 100);

        builder.add(
            ForceType::Newton {
                nx: -0.02,
                ny: -0.01,
            },
            50,
        );

        builder.add(
            ForceType::Accelerate {
                vx: -0.1,
                vy: -0.01,
                vx_max: -0.5,
                vy_max: -1.5,
            },
            30,
        );

        builder.build()
    }

    #[test]
    fn create_grid() {
        let grid = default_grid();
        assert_eq!(grid.cell_width, 100); // 10 * 10
        assert_eq!(grid.cell_height, 100);
        assert_eq!(grid.possibility_spots.len(), 100); // 10 * 10
        assert_eq!(grid.width, 500.);
        assert_eq!(grid.height, 500.);
        assert_eq!(grid.position.x, 1.);
        assert_eq!(grid.position.y, 2.);
    }

    #[test]
    fn fill_grid() {
        let mut grid = default_grid();
        let attributes = default_attributes();

        grid.fill(&attributes, 200, FillStyle::WhiteNoise);

        assert_eq!(grid.particle_count, 200);

        let len = grid
            .possibility_spots
            .iter()
            .fold(0, |acc, x| acc + x.len()) as u32;

        assert_eq!(grid.particle_count, len);
    }

    #[test]
    fn updates_frame() {
        let mut grid = default_grid();
        assert_eq!(0, grid.frame);
        grid.draw();
        assert_eq!(1, grid.frame);
    }

    #[test]
    fn add_particle() {
        let mut grid = default_grid();
        let attributes = default_attributes();

        grid.add_particle(115., 105., &attributes);
        assert_eq!(1, grid.possibility_spots[1].len());

        // if y is 1 more, then the pos in array is + poss_x_count (10).
        grid.add_particle(105., 115., &attributes);
        assert_eq!(1, grid.possibility_spots[10].len());

        let particle = &grid.possibility_spots[1][0];

        assert_eq!(1, grid.possibility_x_index(particle.x));
        assert_eq!(0, grid.possibility_y_index(particle.y));

        assert_eq!(1, grid.cell_x_index(particle.x));
        assert_eq!(1, grid.cell_y_index(particle.y));

        // colors is a number between 0 - 1, (255 / 255).
        assert_eq!(1., particle.color.a);

        assert_eq!(5., particle.diameter);
        assert_eq!(2.5, particle.radius);
        assert_eq!(0.5, particle.friction);
        assert_eq!(0.9, particle.elasticity);
        assert_eq!(1., particle.mass);
    }

    #[macroquad::test]
    async fn moves_particle() {
        let mut grid = default_grid();
        let attributes = default_attributes();

        grid.add_particle(5., 5., &attributes);

        assert_eq!(1, grid.possibility_spots[0].len());

        grid.possibility_spots[0][0].vx += 10.;
        grid.draw();

        assert_eq!(1, grid.possibility_spots[1].len());
        assert_eq!(0, grid.possibility_spots[0].len());
    }

    #[test]
    fn add_external_forces() {
        let forces = default_forces();

        assert_eq!(forces.len(), 4);
        assert_eq!(forces.last().unwrap().until_frame, 230);
    }

    #[test]
    fn apply_forces() {
        // TODO
        let attributes = default_attributes();
        let forces = default_forces();

        let options = GridOptions {
            cell_x_count: 5,
            cell_y_count: 5,
            possibility_x_count: 10,
            possibility_y_count: 10,
            possibility_side_length: 10,
            position: Position::new(1., 2.),
            force_handler: forces,
        };

        let mut grid = Grid::new(options);
    }
}
