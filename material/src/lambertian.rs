use rand::RngExt;

use util::{Color, HitResult, Ray, THREAD_RNG, Unnormalized, Vec3};

use crate::{material_trait::Material, texture::Texture};

pub trait Albedo {
    fn sample(&self, hit: &HitResult) -> Color;
}

impl Albedo for Color {
    fn sample(&self, _hit: &HitResult) -> Color {
        *self
    }
}

impl Albedo for Texture {
    #[allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]
    fn sample(&self, hit: &HitResult) -> Color {
        let x = (hit.u * self.width as f32) as usize % self.width;
        let y = (hit.v * self.height as f32) as usize % self.height;
        self.data[y * self.width + x]
    }
}

#[derive(Debug)]
pub struct LambertianBase<TAlbedo, TORM> {
    pub name: String,
    pub albedo: TAlbedo,
    pub normal_texture: Option<Texture>,
    pub orm: TORM,
    pub alpha: f32,
}

impl<TAlbedo: Albedo + Sync + Send, TORM: Albedo + Sync + Send> Material
    for LambertianBase<TAlbedo, TORM>
{
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

        let orm = self.orm.sample(hit);
        let roughness = orm.y;
        let metallic = orm.z;
        let cos_theta = (-ray.dir.dot(&shading_normal)).max(0.0);
        let fresnel = schlick(cos_theta, 1.5);

        let reflect_prob = f32::max(fresnel, metallic);
        let mut scatter_direction =
            if THREAD_RNG.with(|rng| rng.borrow_mut().random::<f32>()) < reflect_prob {
                ray.dir.reflect(&shading_normal)
            } else {
                (shading_normal + Vec3::<Unnormalized>::random_in_unit_sphere()).normalize()
            };

        // Apply roughness to whichever scatter model was chosen
        if roughness > 0.0 {
            loop {
                scatter_direction = (scatter_direction
                    + Vec3::<Unnormalized>::random_in_unit_sphere() * roughness)
                    .normalize();
                if scatter_direction.dot(&hit.normal) > 0.0 {
                    break;
                }
            }
        }

        // Remove shadow acne
        let origin = hit.point + hit.normal * (hit.t * 1e-4).max(1e-4);
        let scattered = Ray::new(origin, scatter_direction);
        (scattered, self.albedo.sample(hit))
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl From<LambertianBase<Color, Color>> for LambertianBase<Texture, Texture> {
    fn from(base: LambertianBase<Color, Color>) -> Self {
        let pixels = vec![base.albedo];
        let orm = vec![base.orm];

        Self {
            name: base.name,
            albedo: Texture {
                data: pixels,
                width: 1,
                height: 1,
            },
            normal_texture: None,
            orm: Texture {
                data: orm,
                width: 1,
                height: 1,
            },
            alpha: base.alpha,
        }
    }
}

fn schlick(cosine: f32, ior: f32) -> f32 {
    let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
