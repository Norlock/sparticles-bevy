use crate::forces::force::Force;
use crate::forces::force::ForceData;

pub struct LerpForce {
    pub min_nx: f32,
    pub min_ny: f32,
    pub min_nz: f32,
    pub max_nx: f32,
    pub max_ny: f32,
    pub max_nz: f32,
    pub from_ms: u128,
    pub until_ms: u128,
}

const MS_PER_SEC: f32 = 1000.;

impl Force for LerpForce {
    fn apply(&self, data: &mut ForceData, force_cycle_ms: u128) {
        if force_cycle_ms < self.from_ms || self.until_ms <= force_cycle_ms {
            return;
        }

        let delta_current = force_cycle_ms - self.from_ms;
        let delta_max = self.until_ms - self.from_ms;

        let fraction = delta_current as f32 / delta_max as f32;
        let velocity = &mut data.velocity;

        velocity.vx += (self.min_nx + fraction * (self.max_nx - self.min_nx)) / data.mass;
        velocity.vy += (self.min_ny + fraction * (self.max_ny - self.min_ny)) / data.mass;
        velocity.vz += (self.min_nz + fraction * (self.max_nz - self.min_nz)) / data.mass;
    }
}
