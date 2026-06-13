use util::{Interval, Point, Ray};

use crate::{Hittable, HittableType};

pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    pub fn expand_to_contain(&mut self, boundable: &HittableType) {
        let Bounds { min, max } = boundable.get_bounds();
        self.min = Point::min(&self.min, min);
        self.max = Point::max(&self.max, max);
    }

    pub fn longest_axis(&self) -> Axis {
        let diag = self.max - self.min;
        if diag.x > diag.y && diag.x > diag.z {
            Axis::X
        } else if diag.y > diag.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    pub fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Interval> {
        let mut t_min = interval.min;
        let mut t_max = interval.max;

        // X slab
        let mut t0 = (self.min.x - ray.origin.x) * ray.inv_dir.x;
        let mut t1 = (self.max.x - ray.origin.x) * ray.inv_dir.x;
        if ray.inv_dir.x < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return None;
        }

        // Y slab
        let mut t0 = (self.min.y - ray.origin.y) * ray.inv_dir.y;
        let mut t1 = (self.max.y - ray.origin.y) * ray.inv_dir.y;
        if ray.inv_dir.y < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return None;
        }

        // Z slab
        let mut t0 = (self.min.z - ray.origin.z) * ray.inv_dir.z;
        let mut t1 = (self.max.z - ray.origin.z) * ray.inv_dir.z;
        if ray.inv_dir.z < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max < t_min {
            None
        } else {
            Some(Interval {
                min: t_min,
                max: t_max,
            })
        }
    }
}

impl From<&Vec<HittableType>> for Bounds {
    fn from(objects: &Vec<HittableType>) -> Self {
        let mut bounds = Bounds {
            min: Point::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Point::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        };
        for obj in objects {
            bounds.expand_to_contain(obj);
        }
        bounds
    }
}
