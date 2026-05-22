use crate::{
    material::{emissive::Emissive, lambertian::LambertianBase},
    util::{hit_result::HitResult, ray::Ray, vec3::Color},
};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitResult) -> (Ray, Color);
    fn get_name(&self) -> &str;
}

#[allow(dead_code)]
pub enum MaterialType {
    Lambertian(LambertianBase<Color>),
    TextureLambertian(LambertianBase<Vec<Color>>),
    Emissive(Emissive),
}

impl Material for MaterialType {
    fn scatter(&self, ray: &Ray, hit_record: &HitResult) -> (Ray, Color) {
        match self {
            MaterialType::Lambertian(mat) => mat.scatter(ray, hit_record),
            MaterialType::TextureLambertian(mat) => mat.scatter(ray, hit_record),
            MaterialType::Emissive(mat) => mat.scatter(ray, hit_record),
        }
    }

    fn get_name(&self) -> &str {
        match self {
            MaterialType::Lambertian(mat) => mat.get_name(),
            MaterialType::TextureLambertian(mat) => mat.get_name(),
            MaterialType::Emissive(mat) => mat.get_name(),
        }
    }
}
