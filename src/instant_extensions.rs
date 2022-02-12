use std::time::Instant;

impl Elapsed for Instant {
    fn elapsed_ms(&self) -> u128 {
        self.elapsed().as_millis()
    }
}

pub trait Elapsed {
    fn elapsed_ms(&self) -> u128;
}
