use rand::RngExt;

use crate::{
    material::material_trait::Material,
    util::{
        hit_result::HitResult,
        ray::Ray,
        vec3::{Color, THREAD_RNG, Vec3},
    },
};

pub trait Albedo {
    fn sample(&self, hit: &HitResult) -> Color;
}

impl Albedo for Color {
    fn sample(&self, _hit: &HitResult) -> Color {
        *self
    }
}

impl Albedo for Vec<Color> {
    fn sample(&self, hit: &HitResult) -> Color {
        let width = (self.len() as f32).sqrt() as usize;
        let height = self.len() / width;
        let x = (hit.u * width as f32) as usize % width;
        let y = (hit.v * height as f32) as usize % height;
        self[y * width + x]
    }
}

#[derive(Debug)]
pub struct LambertianBase<T> {
    pub name: String,
    pub albedo: T,
    pub normal_texture: Option<Vec<Vec3>>,
    pub roughness: f32,
    pub alpha: f32,
}

impl<T: Albedo + Sync + Send> Material for LambertianBase<T> {
    fn scatter(&self, ray: &Ray, hit: &HitResult) -> (Ray, Color) {
        if self.alpha < 1.0 {
            let transparency_decision = THREAD_RNG.with(|rng| {
                let mut rng = rng.borrow_mut();
                if rng.random::<f32>() < 1f32 - self.alpha {
                    Some((Ray::new(hit.point, ray.dir), Color::new(1.0, 1.0, 1.0)))
                } else {
                    None
                }
            });
            if let Some(result) = transparency_decision {
                return result;
            }
        }

        // Sample normal map and transform to world space
        let shading_normal = if let Some(normal_map) = &self.normal_texture
            && let Some((t, b)) = hit.tangent
        {
            // Sample the normal map (RGB → XYZ in tangent space)
            let sampled = normal_map.sample(hit); // gives [0,1] RGB
            let tangent_normal = Vec3::new(
                sampled.x * 2.0 - 1.0,
                sampled.y * 2.0 - 1.0,
                sampled.z * 2.0 - 1.0,
            )
            .normalize();

            // Transform from tangent space to world space using TBN
            let n = hit.normal;
            (t * tangent_normal.x + b * tangent_normal.y + n * tangent_normal.z).normalize()
        } else {
            hit.normal
        };

        let mut scatter_direction = ray.dir.reflect(shading_normal).normalize();
        if self.roughness > 0.0 {
            loop {
                scatter_direction = (scatter_direction
                    + Vec3::random_in_unit_sphere() * self.roughness)
                    .normalize();
                if scatter_direction.dot(hit.normal) > 0.0 {
                    break;
                }
            }
        }

        // Remove shadow acne
        let origin = hit.point + hit.normal * 1e-4;
        let scattered = Ray::new(origin, scatter_direction);
        (scattered, self.albedo.sample(hit))
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
impl From<LambertianBase<Color>> for LambertianBase<Vec<Color>> {
    fn from(base: LambertianBase<Color>) -> Self {
        let pixels = vec![base.albedo];
        Self {
            name: base.name,
            albedo: pixels,
            normal_texture: None,
            roughness: base.roughness,
            alpha: base.alpha,
        }
    }
}
