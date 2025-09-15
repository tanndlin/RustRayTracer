use crate::{
    material::material_trait::Material,
    util::{hit_result::HitResult, ray::Ray, vec3::Vec3},
};

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> (Ray, Vec3) {
        // let scatter_direction = hit.normal.add(Vec3::random_unit_vector());
        let scatter_direction = ray.dir.reflect(hit.normal);

        let scattered = Ray::new(hit.point, scatter_direction);
        let attenuation = self.albedo;

        (scattered, attenuation)
    }
}
