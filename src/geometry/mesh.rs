use crate::{
    geometry::{aabb::AABB, bounds::Bounds, hittable::Hittable, tri::Tri},
    util::{
        hit_result::HitResult,
        interval::Interval,
        parser::glb::gltf::{self, GltfData},
        ray::Ray,
        vec3::Vec3,
    },
};

pub struct Mesh {
    pub aabb: AABB<Tri>,
}

impl Mesh {
    pub fn new(children: Vec<Tri>) -> Self {
        let aabb = AABB::new(children);
        Mesh { aabb }
    }

    pub fn from_gltf_mesh(gltf_mesh: &gltf::Mesh, gltf_data: &GltfData, binary: &[u8]) -> Self {
        let mut children = Vec::new();

        for primitive in gltf_mesh.primitives {}

        Mesh::new(children)
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        self.aabb.hit(ray, interval)
    }

    fn get_bounds(&self) -> &Bounds {
        self.aabb.get_bounds()
    }

    fn translate(&mut self, vec: &Vec3) {
        self.aabb.translate(vec);
    }
}
