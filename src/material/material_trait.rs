use crate::util::{hit_result::HitResult, ray::Ray, vec3::Color};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitResult) -> (Ray, Color);
    fn get_name(&self) -> &str;
}
