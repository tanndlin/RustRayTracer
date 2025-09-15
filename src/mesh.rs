use crate::{hittable::Hittable, ray::Ray, tri::Tri};

pub struct Mesh {
    pub tris: Vec<Tri>,
}

impl Mesh {
    pub fn new(tris: Vec<Tri>) -> Self {
        Mesh { tris }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray) -> bool {
        self.tris.iter().any(|t| t.hit(ray))
    }
}
