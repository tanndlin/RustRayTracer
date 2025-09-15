use crate::{
    geometry::{bounds::Bounds, hittable::Hittable},
    util::{hit_result::HitResult, ray::Ray},
};

pub struct Mesh<T: Hittable> {
    pub children: Vec<T>,
}

impl<T: Hittable> Mesh<T> {
    pub fn new(children: Vec<T>) -> Self {
        Mesh { children }
    }

    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        self.children
            .iter()
            .filter_map(|t| t.hit(ray))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal))
    }

    fn get_bounds(&self) -> Bounds {
        let mut bounds = self.children[0].get_bounds();
        for tri in &self.children[1..] {
            bounds.expand_to_contain(&tri.get_bounds());
        }

        bounds
    }
}
