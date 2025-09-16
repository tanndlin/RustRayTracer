use crate::util::{hit_result::HitResult, ray::Ray, vec3::Color};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitResult) -> (Ray, Color);
}
