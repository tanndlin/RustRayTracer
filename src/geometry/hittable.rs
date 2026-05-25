use crate::{
    geometry::{bounds::Bounds, instance::Instance, mesh::Mesh, sphere::Sphere},
    util::{hit_result::HitResult, interval::Interval, parser::glb::gltf, ray::Ray, vec3::Vec3},
};

#[allow(dead_code)]
pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult>;
    fn get_bounds(&self) -> &Bounds;
    fn translate(&mut self, vec: &Vec3);
    fn scale(&mut self, vec: &Vec3);
    fn rotate(&mut self, axis: &Vec3, angle_rad: f32);
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum HittableType {
    Sphere(Sphere),
    Mesh(Mesh),
    Instance(Instance),
}

impl Hittable for HittableType {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        match self {
            HittableType::Sphere(sphere) => sphere.hit(ray, interval),
            HittableType::Mesh(mesh) => mesh.hit(ray, interval),
            HittableType::Instance(instance) => instance.hit(ray, interval),
        }
    }

    fn get_bounds(&self) -> &Bounds {
        match self {
            HittableType::Sphere(sphere) => sphere.get_bounds(),
            HittableType::Mesh(mesh) => mesh.get_bounds(),
            HittableType::Instance(instance) => instance.get_bounds(),
        }
    }

    fn translate(&mut self, vec: &Vec3) {
        match self {
            HittableType::Sphere(sphere) => sphere.translate(vec),
            HittableType::Mesh(mesh) => mesh.translate(vec),
            HittableType::Instance(instance) => instance.translate(vec),
        }
    }

    fn scale(&mut self, vec: &Vec3) {
        match self {
            HittableType::Sphere(sphere) => sphere.scale(vec),
            HittableType::Mesh(mesh) => mesh.scale(vec),
            HittableType::Instance(instance) => instance.scale(vec),
        }
    }

    fn rotate(&mut self, axis: &Vec3, angle_rad: f32) {
        match self {
            HittableType::Sphere(sphere) => sphere.rotate(axis, angle_rad),
            HittableType::Mesh(mesh) => mesh.rotate(axis, angle_rad),
            HittableType::Instance(instance) => instance.rotate(axis, angle_rad),
        }
    }
}

impl HittableType {
    pub fn from_gltf_mesh(
        gltf_mesh: &gltf::Mesh,
        gltf_data: &gltf::GltfData,
        binary: &[u8],
        mat_offset: usize,
    ) -> Self {
        HittableType::Mesh(Mesh::from_gltf_mesh(
            gltf_mesh, gltf_data, binary, mat_offset,
        ))
    }
}
