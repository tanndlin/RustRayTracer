use crate::{
    geometry::{bounds::Bounds, hittable::Hittable},
    util::{hit_result::HitResult, interval::Interval, ray::Ray, vec3::Vec3},
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    bounds: Bounds,
    pub material_index: usize,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material_index: usize) -> Self {
        let r_vec = Vec3 {
            x: radius,
            y: radius,
            z: radius,
        };

        let bounds = Bounds {
            min: center - r_vec,
            max: center + r_vec,
        };

        Sphere {
            center,
            radius,
            bounds,
            material_index,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t1 = (h - sqrt_d) / a;
        let t2 = (h + sqrt_d) / a;
        let t = if t1 >= 0.0 { t1 } else { t2 };
        if t < 1e-6 || !interval.contains(t) {
            return None;
        }

        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;

        let u = 0.5 + (normal.z.atan2(normal.x)) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - (normal.y.asin()) / std::f32::consts::PI;

        Some(HitResult {
            normal,
            t,
            point,
            u,
            v,
            material_index: self.material_index,
        })
    }

    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn translate(&mut self, vec: &Vec3) {
        self.center = self.center + *vec;
    }
}
