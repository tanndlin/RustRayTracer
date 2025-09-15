use crate::{
    util::ray::Ray,
    util::vec3::{Vec3, max, min},
};

pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Bounds {
    pub(crate) fn expand_to_contain(&mut self, get_bounds: &Bounds) {
        self.min = min(self.min, get_bounds.min);
        self.max = max(self.max, get_bounds.max);
    }

    pub fn longest_axis(&self) -> Axis {
        let diag = self.max.sub(self.min);
        if diag.x > diag.y && diag.x > diag.z {
            Axis::X
        } else if diag.y > diag.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        // Slab method
        let t0s = self.min.sub(ray.origin).mul(ray.inv_dir);
        let t1s = self.max.sub(ray.origin).mul(ray.inv_dir);

        let tsmalls = Vec3 {
            x: t0s.x.min(t1s.x),
            y: t0s.y.min(t1s.y),
            z: t0s.z.min(t1s.z),
        };
        let tbigs = Vec3 {
            x: t0s.x.max(t1s.x),
            y: t0s.y.max(t1s.y),
            z: t0s.z.max(t1s.z),
        };

        let tmin = tsmalls.x.max(tsmalls.y).max(tsmalls.z);
        let tmax = tbigs.x.min(tbigs.y).min(tbigs.z);

        if tmax >= tmin.max(0.0) && tmax > 0.0 {
            Some(tmin)
        } else {
            None
        }
    }
}
