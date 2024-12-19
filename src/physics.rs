use ultraviolet::DVec3;
use crate::cdm::Cdm;

pub const MIN_DISTANCE: f64 = 0.1;

pub fn calculate_gravity(p1: &Cdm, p2: &Cdm) -> DVec3 {
    calculate_gravity_with_com(p1, p2.mass, p2.pos)
}

pub fn calculate_gravity_with_com(p: &Cdm, mass: f64, pos: DVec3) -> DVec3 {
    let r = pos - p.pos;
    let r_mag_sq = r.mag_sq().max(MIN_DISTANCE * MIN_DISTANCE);
    let r_mag = r_mag_sq.sqrt();
    let force = p.mass * mass / r_mag_sq;
    r.normalized() * force
}