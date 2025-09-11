use crate::{
    hittable::Hittable,
    ray::{Ray, dot},
    vec3::Vec3,
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
}
