use crate::{
    material::material_trait::Material,
    util::{hit_result::HitResult, ray::Ray, vec3::Color},
};

pub struct Emissive {
    pub name: String,
    pub intensity: f32,
    pub color: Color,
}

impl Material for Emissive {
    fn scatter(&self, ray: &Ray, hit_record: &HitResult) -> (Ray, Color) {
        (
            Ray::new(hit_record.point, ray.dir),
            self.color * self.intensity,
        )
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
