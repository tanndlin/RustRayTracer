use crate::{bounds::Bounds, ray::Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> bool;
    fn get_bounds(&self) -> Bounds;
}
