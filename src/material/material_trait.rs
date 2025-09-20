use crate::{
    material::lambertian::{Lambertian, TextureLambertian},
    util::{hit_result::HitResult, ray::Ray, vec3::Color},
};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitResult) -> (Ray, Color);
    fn get_name(&self) -> &str;
}

pub enum MaterialType {
    Lambertian(Lambertian),
    TextureLambertian(TextureLambertian),
}

impl Material for MaterialType {
    fn scatter(&self, ray: &Ray, hit_record: &HitResult) -> (Ray, Color) {
        match self {
            MaterialType::Lambertian(mat) => mat.scatter(ray, hit_record),
            MaterialType::TextureLambertian(mat) => mat.scatter(ray, hit_record),
        }
    }

    fn get_name(&self) -> &str {
        match self {
            MaterialType::Lambertian(mat) => mat.get_name(),
            MaterialType::TextureLambertian(mat) => mat.get_name(),
        }
    }
}
