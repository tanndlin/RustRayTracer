use crate::{
    geometry::bounds::Bounds,
    util::{hit_result::HitResult, interval::Interval, ray::Ray},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult>;
    fn get_bounds(&self) -> Bounds;
}
