use crate::{
    material::material_trait::Material,
    util::{hit_result::HitResult, ray::Ray, vec3::Color},
};

#[derive(Debug)]
pub struct Lambertian {
    pub name: String,
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> (Ray, Color) {
        let scatter_direction = ray.dir.reflect(hit.normal);

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
}

impl Material for TextureLambertian {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> (Ray, Color) {
        let scatter_direction = ray.dir.reflect(hit.normal);

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
