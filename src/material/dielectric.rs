use rand::RngExt;

use crate::{
    material::material_trait::Material,
    util::{
        hit_result::HitResult,
        ray::Ray,
        vec3::{Color, THREAD_RNG, Vec3},
    },
};

pub struct Dielectric {
    name: String,
    albedo: Color,
    refraction_index: f32,
    transmission_factor: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitResult) -> (Ray, Color) {
        if random_double() > self.transmission_factor {
            // Treat as opaque lambertian
            let scatter_dir = hit_record.normal + Vec3::random_in_unit_sphere().normalize();
            return (Ray::new(hit_record.point, scatter_dir), self.albedo);
        }

        let ri = match hit_record.front_face {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let unit_dir = ray.dir.normalize();
        let cos_theta = f32::min(-unit_dir.dot(hit_record.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let dir = match cannot_refract || reflectance(cos_theta, ri) > random_double() {
            true => self.reflect(unit_dir, hit_record.normal),
            false => self.refract(unit_dir, hit_record.normal, ri),
        };

        let origin = hit_record.point + dir * 1e-4;
        let new_ray = Ray::new(origin, dir);

        (new_ray, self.albedo)
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Dielectric {
    pub fn new(
        name: String,
        albedo: Option<Color>,
        refraction_index: f32,
        transmission_factor: f32,
    ) -> Self {
        Self {
            name,
            albedo: albedo.unwrap_or(Color::new(1.0, 1.0, 1.0)),
            refraction_index,
            transmission_factor,
        }
    }

    fn reflect(&self, v: Color, n: Color) -> Color {
        v - n * 2.0 * v.dot(n)
    }

    fn refract(&self, uv: Color, n: Color, ri: f32) -> Color {
        let cos_theta = f32::min(-uv.dot(n), 1.0);
        let r_out_perp = (uv + n * cos_theta) * ri;
        let r_out_parallel = n * -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared()));
        r_out_perp + r_out_parallel
    }
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
}

fn random_double() -> f32 {
    THREAD_RNG.with(|rng| rng.borrow_mut().random::<f32>())
}
