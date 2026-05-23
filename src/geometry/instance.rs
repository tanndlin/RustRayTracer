use std::sync::Arc;

use crate::{
    geometry::{
        bounds::Bounds,
        hittable::{Hittable, HittableType},
    },
    util::{
        hit_result::HitResult, interval::Interval, parser::glb::gltf::Node, ray::Ray, vec3::Vec3,
    },
};

#[derive(Debug)]
pub struct Instance {
    pub name: String,
    pub translation: Vec3,
    pub base: Arc<HittableType>,
}

impl Instance {
    pub fn new(name: String, translation: Vec3, base: Arc<HittableType>) -> Self {
        Self {
            name,
            translation,
            base,
        }
    }
}

impl Hittable for Instance {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        let translated_ray = Ray::new(ray.origin - self.translation, ray.dir);

        let mut hit_result = self.base.hit(&translated_ray, interval)?;
        hit_result.point = hit_result.point + self.translation;

        Some(hit_result)
    }

    fn get_bounds(&self) -> &Bounds {
        todo!()
    }

    fn translate(&mut self, vec: &Vec3) {
        self.translation = self.translation + *vec;
    }
}

impl From<(&[Arc<HittableType>], Node)> for Instance {
    fn from(value: (&[Arc<HittableType>], Node)) -> Self {
        let (meshes, node) = value;
        let mesh_index = node
            .mesh
            .expect("GLTF node must have a mesh to be instanced");
        let translation = Vec3::from(
            node.translation
                .clone()
                .unwrap_or(vec![0.0, 0.0, 0.0])
                .as_slice(),
        );
        let base = meshes
            .get(mesh_index)
            .expect("Mesh index out of bounds for GLTF node")
            .clone();

        Self::new(node.name, translation, base)
    }
}
