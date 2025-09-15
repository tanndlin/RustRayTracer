use crate::{
    geometry::bounds::Bounds,
    util::{hit_result::HitResult, ray::Ray},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitResult>;
    fn get_bounds(&self) -> Bounds;
}
