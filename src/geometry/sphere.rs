use crate::{
    geometry::{bounds::Bounds, hittable::Hittable},
    util::{
        hit_result::HitResult,
        ray::Ray,
        vec3::{Vec3, dot},
    },
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        let oc = self.center.sub(ray.origin);
        let a = ray.dir.length_squared();
        let h = dot(ray.dir, oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t1 = (h - sqrt_d) / a;
        let t2 = (h + sqrt_d) / a;
        let t = if t1 >= 0.0 { t1 } else { t2 };
        if t < 0.0 {
            return None;
        }

        let point = ray.at(t);
        let normal = point.sub(self.center).scale(1.0 / self.radius);
        Some(HitResult { normal, t })
    }

    fn get_bounds(&self) -> Bounds {
        let r_vec = Vec3 {
            x: self.radius,
            y: self.radius,
            z: self.radius,
        };

        Bounds {
            min: self.center.sub(r_vec),
            max: self.center.add(r_vec),
        }
    }
}
