use crate::force::force::{Force, ForceData};

pub struct ConstantForce {
    pub nx: f32,
    pub ny: f32,
    pub max_vx: f32,
    pub max_vy: f32,
    pub from_ms: u128,
    pub until_ms: u128,
}

const MS_PER_SEC: f32 = 1000.;

impl Force for ConstantForce {
    fn apply(&self, data: &mut ForceData, force_cycle_ms: u128) {
        if force_cycle_ms < self.from_ms || self.until_ms <= force_cycle_ms {
            return;
        }

        let vx = self.nx / data.mass;
        let vy = self.ny / data.mass;

        let new_vx = data.vx + vx;
        let new_vy = data.vy + vy;

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
    }
}
