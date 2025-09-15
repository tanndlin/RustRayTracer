use crate::util::{hit_result::HitResult, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitResult) -> (Ray, Vec3);
}
