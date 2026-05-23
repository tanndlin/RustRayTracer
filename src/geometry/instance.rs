use std::sync::Arc;

use crate::{
    geometry::{
        bounds::Bounds,
        hittable::{Hittable, HittableType},
    },
    util::{hit_result::HitResult, interval::Interval, ray::Ray, vec3::Vec3},
};

pub struct Instance {
    pub mesh_index: usize,
    pub translation: Vec3,
    pub base: Arc<HittableType>,
}

impl<'a> Instance {
    pub fn new(mesh_index: usize, translation: Vec3, base: Arc<HittableType>) -> Self {
        Self {
            mesh_index,
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
