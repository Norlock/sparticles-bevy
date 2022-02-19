use crate::emitters::emitter::Velocity;
use rand::{thread_rng, Rng};

pub fn equalize_total_speed(speed_squared: f32, new: &mut Velocity) {
    let new_vx_squared = new.vx.powi(2);
    let new_vy_squared = new.vy.powi(2);
    let new_vz_squared = new.vz.powi(2);
    let new_speed_squared = new_vx_squared + new_vy_squared + new_vz_squared;

    let scale_factor = speed_squared / new_speed_squared;
    new.vx = (new_vx_squared * scale_factor).sqrt() * new.vx.signum();
    new.vy = (new_vy_squared * scale_factor).sqrt() * new.vy.signum();
    new.vz = (new_vz_squared * scale_factor).sqrt() * new.vz.signum();
}

pub fn stray_velocity(velocity: &mut Velocity, stray_radians: f32) {
    let speed_squared = velocity.vy.powi(2) + velocity.vz.powi(2) + velocity.vx.powi(2);
    let speed = speed_squared.sqrt();

    let mut rng = thread_rng();
    let mut stray_factor = || rng.gen_range(-stray_radians..stray_radians) / 2.;

    let cos_x = velocity.vx / speed + stray_factor();
    let cos_y = velocity.vy / speed + stray_factor();
    let cos_z = velocity.vz / speed + stray_factor();

    let new_vx = speed * cos_x;
    let new_vy = speed * cos_y;
    let new_vz = speed * cos_z;

    let mut new_velocity = Velocity::new(new_vx, new_vy, new_vz);

    equalize_total_speed(speed_squared, &mut new_velocity);

    velocity.vx = new_velocity.vx;
    velocity.vy = new_velocity.vy;
    velocity.vz = new_velocity.vz;
}
