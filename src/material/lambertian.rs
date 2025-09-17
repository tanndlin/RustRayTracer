use crate::{
    material::material_trait::Material,
    util::{
        hit_result::HitResult,
        ray::Ray,
        vec3::{Color, Vec3, dot},
    },
};

#[derive(Debug)]
pub struct Lambertian {
    pub name: String,
    pub albedo: Color,
    pub roughness: f64,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> (Ray, Color) {
        let mut scatter_direction = ray.dir.reflect(hit.normal).normalize();
        // Add some randomness to the scatter direction based on roughness
        if self.roughness > 0.0 {
            loop {
                scatter_direction = (scatter_direction
                    + Vec3::random_in_unit_sphere() * self.roughness)
                    .normalize();
                if dot(scatter_direction, hit.normal) > 0.0 {
                    break;
                }
            }
        }

        let scattered = Ray::new(hit.point, scatter_direction);
        let attenuation = self.albedo;

        (scattered, attenuation)
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct TextureLambertian {
    pub name: String,
    pub pixels: Vec<Color>,
    pub roughness: f64,
}

impl Material for TextureLambertian {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> (Ray, Color) {
        let mut scatter_direction = ray.dir.reflect(hit.normal).normalize();
        if self.roughness > 0.0 {
            loop {
                scatter_direction = (scatter_direction
                    + Vec3::random_in_unit_sphere() * self.roughness)
                    .normalize();
                if dot(scatter_direction, hit.normal) > 0.0 {
                    break;
                }
            }
        }

        let u = hit.u;
        let v = hit.v;
        let width = ((self.pixels.len() as f32).sqrt()) as usize;
        let height = width; // Assuming square texture for simplicity
        let x = (u * width as f64) as usize % width;
        let y = (v * height as f64) as usize % height;
        let pixel_index = y * width + x;
        let attenuation = self.pixels[pixel_index];

        let scattered = Ray::new(hit.point, scatter_direction);

        (scattered, attenuation)
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
