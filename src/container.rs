use core::fmt::Formatter;
use std::fmt::Debug;

use crate::{grid::Grid, particle::Particle};

pub trait Container {
    fn add_particle(&mut self, particle: &Particle);
    fn remove_particle(&self, particle: &Particle);
    fn start_render(&self);
    fn stop_render(&self);
    fn draw_dev_grid(&self, grid: &Grid);
    fn show_dev_grid(&self);
    fn hide_dev_grid(&self);
}

impl Debug for dyn Container {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Nothing
        Ok(())
    }
}
