use ultraviolet::DVec3;

#[derive(Clone, Debug)]
pub struct Cdm {
    pub pos: DVec3,
    pub vel: DVec3,
    pub acc: DVec3,
    pub mass: f64,
}

impl Cdm {
    pub fn new(pos: DVec3, vel: DVec3, mass: f64) -> Self {
        Self { 
            pos,
            vel,
            acc: DVec3::zero(),
            mass
        }
    }
    
    pub fn update(&mut self, dt: f64) {
        self.pos += (self.vel * dt);
        self.vel += (self.acc * dt);
        self.acc = DVec3::zero();
    }
}