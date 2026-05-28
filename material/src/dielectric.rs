use rand::RngExt;
use util::{
    HitResult, Ray, {Color, THREAD_RNG, Vec3},
};

use crate::material_trait::Material;

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
            let mut scatter_dir = hit_record.normal + Vec3::random_in_unit_sphere().normalize();
            if scatter_dir.dot(hit_record.normal) < 0.0 {
                scatter_dir = -scatter_dir;
            }
            let origin = hit_record.point + hit_record.normal * 1e-4;
            return (Ray::new(origin, scatter_dir), self.albedo);
        }

        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = ray.dir.normalize();
        let cos_theta = f32::min(-unit_dir.dot(hit_record.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let dir = if cannot_refract || reflectance(cos_theta, ri) > random_double() {
            Self::reflect(unit_dir, hit_record.normal)
        } else {
            Self::refract(unit_dir, hit_record.normal, ri)
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

    fn reflect(v: Color, n: Color) -> Color {
        v - n * 2.0 * v.dot(n)
    }

    fn refract(uv: Color, n: Color, ri: f32) -> Color {
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
