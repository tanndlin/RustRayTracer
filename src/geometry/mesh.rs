use crate::{
    geometry::{bounds::Bounds, hittable::Hittable},
    util::ray::Ray,
};

pub struct Mesh<T: Hittable> {
    pub children: Vec<T>,
}

impl<T: Hittable> Mesh<T> {
    pub fn new(children: Vec<T>) -> Self {
        Mesh { children }
    }
}

impl<T: Hittable> Hittable for Mesh<T> {
    fn hit(&self, ray: &Ray) -> bool {
        self.children.iter().any(|t| t.hit(ray))
    }

    fn get_bounds(&self) -> Bounds {
        let mut bounds = self.children[0].get_bounds();
        for tri in &self.children[1..] {
            bounds.expand_to_contain(&tri.get_bounds());
        }

        bounds
    }
}
