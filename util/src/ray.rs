use crate::{
    Point,
    vec3::{Normalized, Vec3},
};

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3<Normalized>,
    pub inv_dir: Vec3<Normalized>,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3<Normalized>) -> Self {
        Self {
            origin,
            dir,
            inv_dir: dir.invert().normalize(),
        }
    }

    // Projects ray to a certain distance
    pub fn at(&self, dst: f32) -> Point {
        self.origin + self.dir * dst
    }
}
