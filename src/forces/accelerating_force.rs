use super::force::{Force, ForceData};
use bevy::prelude::Component;

/**
 * Builds up applying force form 0 to nx/ny over time.
 * max_(vx/vy) will determin the max (positive or negative) speed a particle in similar direction needs to have the force applied.
 */
#[derive(Debug, Component)]
pub struct AcceleratingForce {
    pub nx: f32,
    pub ny: f32,
    pub nz: f32,
    pub max_vx: f32,
    pub max_vy: f32,
    pub max_vz: f32,
    pub from_ms: u128,
    pub until_ms: u128,
}

const MS_PER_SEC: f32 = 1000.;

impl Force for AcceleratingForce {
    fn apply(&self, data: &mut ForceData, force_cycle_ms: u128) {
        if force_cycle_ms < self.from_ms || self.until_ms <= force_cycle_ms {
            return;
        }

        let acceleration = ((force_cycle_ms - self.from_ms) as f32 / MS_PER_SEC).powf(2.);
        let vx = self.nx * acceleration / data.mass;
        let vy = self.ny * acceleration / data.mass;
        let vz = self.nz * acceleration / data.mass;

        let new_vx = data.vx + vx;
        let new_vy = data.vy + vy;
        let new_vz = data.vz + vz;

        if 0. < vx && 0. <= data.vx {
            if new_vx <= self.max_vx {
                data.vx += vx;
            }
        } else if vx < 0. && data.vx <= 0. {
            if self.max_vx <= new_vx {
                data.vx += vx;
            }
        } else {
            data.vx += vx;
        }

        if 0. < vy && 0. <= data.vy {
            if new_vy <= self.max_vy {
                data.vy += vy;
            }
        } else if vy < 0. && data.vy <= 0. {
            if self.max_vy <= new_vy {
                data.vy += vy;
            }
        } else {
            data.vy += vy;
        }

        if 0. < vz && 0. <= data.vz {
            if new_vz <= self.max_vz {
                data.vz += vz;
            }
        } else if vz < 0. && data.vz <= 0. {
            if self.max_vz <= new_vz {
                data.vz += vz;
            }
        } else {
            data.vz += vz;
        }
    }
}
