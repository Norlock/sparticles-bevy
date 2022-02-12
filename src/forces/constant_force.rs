use super::force::{Force, ForceData};

pub struct ConstantForce {
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

impl Force for ConstantForce {
    fn apply(&self, data: &mut ForceData, force_cycle_ms: u128) {
        if force_cycle_ms < self.from_ms || self.until_ms <= force_cycle_ms {
            return;
        }

        let velocity = &mut data.velocity;

        let vx = self.nx / data.mass;
        let vy = self.ny / data.mass;
        let vz = self.nz / data.mass;

        let new_vx = velocity.vx + vx;
        let new_vy = velocity.vy + vy;
        let new_vz = velocity.vy + vz;

        if 0. < vx && 0. <= velocity.vx {
            if new_vx <= self.max_vx {
                velocity.vx += vx;
            }
        } else if vx < 0. && velocity.vx <= 0. {
            if self.max_vx <= new_vx {
                velocity.vx += vx;
            }
        } else {
            velocity.vx += vx;
        }

        if 0. < vy && 0. <= velocity.vy {
            if new_vy <= self.max_vy {
                velocity.vy += vy;
            }
        } else if vy < 0. && velocity.vy <= 0. {
            if self.max_vy <= new_vy {
                velocity.vy += vy;
            }
        } else {
            velocity.vy += vy;
        }

        if 0. < vz && 0. <= velocity.vz {
            if new_vz <= self.max_vz {
                velocity.vz += vz;
            }
        } else if vz < 0. && velocity.vz <= 0. {
            if self.max_vz <= new_vz {
                velocity.vz += vz;
            }
        } else {
            velocity.vz += vz;
        }
    }
}
