use crate::{
    geometry::{aabb::AABB, bounds::Bounds, hittable::Hittable},
    util::{hit_result::HitResult, interval::Interval, ray::Ray, vec3::Vec3},
};

pub struct Mesh<T: Hittable> {
    pub aabb: AABB<T>,
}

impl<T: Hittable> Mesh<T> {
    pub fn new(children: Vec<T>) -> Self {
        let aabb = AABB::new(children);
        Mesh { aabb }
    }
}

impl<T: Hittable> Hittable for Mesh<T> {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        self.aabb.hit(ray, interval)
    }

    fn get_bounds(&self) -> Bounds {
        self.aabb.get_bounds()
    }

    fn translate(&mut self, vec: &Vec3) {
        self.aabb.translate(vec);
    }
}
