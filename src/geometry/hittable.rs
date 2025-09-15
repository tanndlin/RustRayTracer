use crate::{geometry::bounds::Bounds, util::ray::Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> bool;
    fn get_bounds(&self) -> Bounds;
}
