#![allow(clippy::many_single_char_names)]

use util::{HitResult, Interval, Point, Ray, Vec3};

use crate::{bounds::Bounds, hittable::Hittable};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    bounds: Bounds,
    pub material_index: Option<usize>,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Vec3, radius: f32, material_index: Option<usize>) -> Self {
        let r_vec = Point::new(radius, radius, radius);
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
        let h = ray.dir.dot(&oc);
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
        let outward = (point - self.center) / self.radius;

        let u = 0.5 + outward.z.atan2(outward.x) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - outward.y.asin() / std::f32::consts::PI;
        let normal = outward.normalize();

        Some(HitResult {
            normal,
            tangent: None, // TODO
            t,
            point,
            u,
            v,
            material_index: self.material_index,
            front_face: ray.dir.dot(&normal) < 0.0,
        })
    }

    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn translate(&mut self, vec: &Vec3) {
        self.center = self.center + *vec;
    }

    fn scale(&mut self, _vec: &Vec3) {
        todo!()
    }

    fn rotate(&mut self, _axis: &Vec3, _angle_rad: f32) {
        todo!()
    }
}
