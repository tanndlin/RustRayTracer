use crate::{
    geometry::{bounds::Bounds, mesh::Mesh, sphere::Sphere},
    util::{hit_result::HitResult, interval::Interval, ray::Ray, vec3::Vec3},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult>;
    fn get_bounds(&self) -> &Bounds;
    fn translate(&mut self, vec: &Vec3);
}

pub enum HittableType {
    Sphere(Sphere),
    Mesh(Mesh),
}

impl Hittable for HittableType {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        match self {
            HittableType::Sphere(sphere) => sphere.hit(ray, interval),
            HittableType::Mesh(mesh) => mesh.hit(ray, interval),
        }
    }

    fn get_bounds(&self) -> &Bounds {
        match self {
            HittableType::Sphere(sphere) => sphere.get_bounds(),
            HittableType::Mesh(mesh) => mesh.get_bounds(),
        }
    }

    fn translate(&mut self, vec: &Vec3) {
        match self {
            HittableType::Sphere(sphere) => sphere.translate(vec),
            HittableType::Mesh(mesh) => mesh.translate(vec),
        }
    }
}
