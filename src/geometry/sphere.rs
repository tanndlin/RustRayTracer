use crate::{
    geometry::{bounds::Bounds, hittable::Hittable},
    util::ray::Ray,
    util::vec3::{Vec3, dot},
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> bool {
        let oc = self.center.sub(ray.origin);
        let a = ray.dir.length_squared();
        let h = dot(ray.dir, oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        discriminant >= 0.0
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
