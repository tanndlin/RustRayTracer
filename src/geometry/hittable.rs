use crate::{
    geometry::bounds::Bounds,
    util::{hit_result::HitResult, interval::Interval, ray::Ray, vec3::Vec3},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult>;
    fn get_bounds(&self) -> Bounds;
    fn translate(&mut self, vec: &Vec3);
}
