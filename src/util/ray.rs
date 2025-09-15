use crate::util::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub inv_dir: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: Vec3, dir: Vec3) -> Self {
        Self {
            origin,
            dir,
            inv_dir: dir.invert().normalize(),
        }
    }

    // Projects ray to a certain distance
    pub(crate) fn at(&self, dst: f64) -> Vec3 {
        self.origin.add(self.dir.scale(dst))
    }
}
