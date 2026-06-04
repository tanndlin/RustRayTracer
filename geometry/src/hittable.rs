use gltf::{GltfData, GltfMesh};
use util::{HitResult, Interval, Ray, Vec3};

use crate::{bounds::Bounds, instance::Instance, mesh::Mesh, sphere::Sphere, tri::Tri};

#[allow(dead_code)]
pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult>;
    fn get_bounds(&self) -> &Bounds;
    fn translate(&mut self, vec: &Vec3);
    fn scale(&mut self, vec: &Vec3);
    fn rotate(&mut self, axis: &Vec3, angle_rad: f32);
    fn debug_hit_count(&self, _ray: &Ray, _interval: &Interval) -> u32 {
        0
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum HittableType {
    Sphere(Sphere),
    Tri(Tri),
    Mesh(Mesh),
    Instance(Box<Instance>),
}

impl Hittable for HittableType {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        match self {
            HittableType::Sphere(sphere) => sphere.hit(ray, interval),
            HittableType::Tri(tri) => tri.hit(ray, interval),
            HittableType::Mesh(mesh) => mesh.hit(ray, interval),
            HittableType::Instance(instance) => instance.hit(ray, interval),
        }
    }

    fn get_bounds(&self) -> &Bounds {
        match self {
            HittableType::Sphere(sphere) => sphere.get_bounds(),
            HittableType::Tri(tri) => tri.get_bounds(),
            HittableType::Mesh(mesh) => mesh.get_bounds(),
            HittableType::Instance(instance) => instance.get_bounds(),
        }
    }

    fn translate(&mut self, vec: &Vec3) {
        match self {
            HittableType::Sphere(sphere) => sphere.translate(vec),
            HittableType::Tri(tri) => tri.translate(vec),
            HittableType::Mesh(mesh) => mesh.translate(vec),
            HittableType::Instance(instance) => instance.translate(vec),
        }
    }

    fn scale(&mut self, vec: &Vec3) {
        match self {
            HittableType::Sphere(sphere) => sphere.scale(vec),
            HittableType::Tri(tri) => tri.scale(vec),
            HittableType::Mesh(mesh) => mesh.scale(vec),
            HittableType::Instance(instance) => instance.scale(vec),
        }
    }

    fn rotate(&mut self, axis: &Vec3, angle_rad: f32) {
        match self {
            HittableType::Sphere(sphere) => sphere.rotate(axis, angle_rad),
            HittableType::Tri(tri) => tri.rotate(axis, angle_rad),
            HittableType::Mesh(mesh) => mesh.rotate(axis, angle_rad),
            HittableType::Instance(instance) => instance.rotate(axis, angle_rad),
        }
    }

    fn debug_hit_count(&self, ray: &Ray, interval: &Interval) -> u32 {
        match self {
            HittableType::Sphere(_) | HittableType::Tri(_) => 0,
            HittableType::Mesh(mesh) => mesh.debug_hit_count(ray, interval),
            HittableType::Instance(instance) => instance.debug_hit_count(ray, interval),
        }
    }
}

impl HittableType {
    pub fn from_gltf_mesh(
        gltf_mesh: &GltfMesh,
        gltf_data: &GltfData,
        binary: &[u8],
        mat_offset: usize,
    ) -> Self {
        HittableType::Mesh(Mesh::from_gltf_mesh(
            gltf_mesh, gltf_data, binary, mat_offset,
        ))
    }
}
