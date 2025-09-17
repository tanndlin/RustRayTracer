use crate::{
    geometry::{
        bounds::{Axis, Bounds},
        hittable::Hittable,
    },
    util::{hit_result::HitResult, interval::Interval, ray::Ray, vec3::Vec3},
};

const MIN_CHILDREN: usize = 10;

pub enum AABBType<T> {
    Recursive(RecursiveAABB<T>),
    Leaf(Vec<T>),
}

#[allow(clippy::upper_case_acronyms)]
pub struct AABB<T> {
    pub aabb_type: AABBType<T>,
    pub bounds: Bounds,
}

impl<T: Hittable> AABB<T> {
    pub fn new(children: Vec<T>) -> Self {
        let bounds = Self::calc_bounds(&children);

        let num_children = children.len();
        if num_children <= MIN_CHILDREN {
            return AABB {
                aabb_type: AABBType::Leaf(children),
                bounds,
            };
        }

        let longest_axis = bounds.longest_axis();

        let mut sorted_tris = children;
        sorted_tris.sort_by(|a, b| {
            let a_bounds = a.get_bounds();
            let b_bounds = b.get_bounds();
            let a_center = a_bounds.min + a_bounds.max * 0.5;
            let b_center = b_bounds.min + b_bounds.max * 0.5;

            match longest_axis {
                Axis::X => a_center.x.partial_cmp(&b_center.x).unwrap(),
                Axis::Y => a_center.y.partial_cmp(&b_center.y).unwrap(),
                Axis::Z => a_center.z.partial_cmp(&b_center.z).unwrap(),
            }
        });

        let mid = num_children / 2;
        let left_tris = sorted_tris.split_off(mid);
        let right_tris = sorted_tris;

        let left_aabb = AABB::new(left_tris);
        let right_aabb = AABB::new(right_tris);
        AABB {
            aabb_type: AABBType::Recursive(RecursiveAABB::new(
                Box::new(left_aabb),
                Box::new(right_aabb),
            )),
            bounds,
        }
    }

    fn calc_bounds(tris: &[T]) -> Bounds {
        let mut bounds = tris[0].get_bounds();
        for tri in &tris[1..] {
            bounds.expand_to_contain(&tri.get_bounds());
        }

        bounds
    }
}

impl<T: Hittable> Hittable for AABB<T> {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        self.bounds.hit(ray, interval)?;
        match &self.aabb_type {
            AABBType::Recursive(c) => c.hit(ray, interval),
            AABBType::Leaf(children) => {
                let mut best_hit: Option<HitResult> = None;
                let mut t_max = interval.max;

                for child in children {
                    if let Some(hit) = child.hit(
                        ray,
                        &Interval {
                            min: interval.min,
                            max: t_max,
                        },
                    ) && (best_hit.is_none() || hit.t < best_hit.as_ref().unwrap().t)
                    {
                        best_hit = Some(hit);
                        t_max = best_hit.as_ref().unwrap().t;
                    }
                }

                best_hit
            }
        }
    }

    fn get_bounds(&self) -> Bounds {
        self.bounds
    }

    fn translate(&mut self, vec: &Vec3) {
        self.bounds.min = self.bounds.min + *vec;
        self.bounds.max = self.bounds.max + *vec;

        match &mut self.aabb_type {
            AABBType::Recursive(c) => {
                c.left.translate(vec);
                c.right.translate(vec);
            }
            AABBType::Leaf(children) => {
                for child in children {
                    child.translate(vec);
                }
            }
        }
    }
}

pub struct RecursiveAABB<T> {
    pub left: Box<AABB<T>>,
    pub right: Box<AABB<T>>,
}

impl<T: Hittable> RecursiveAABB<T> {
    pub fn new(left: Box<AABB<T>>, right: Box<AABB<T>>) -> Self {
        Self { left, right }
    }

    pub fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        let left_bounds = self.left.bounds.hit(ray, interval);
        let right_bounds = self.right.bounds.hit(ray, interval);

        match (left_bounds, right_bounds) {
            (Some(t_left), Some(t_right)) => {
                // Decide which child is nearer based on entry distance
                let (first, second, far_bounds) = if t_left.min < t_right.min {
                    (&self.left, &self.right, t_right)
                } else {
                    (&self.right, &self.left, t_left)
                };

                // Traverse near child
                if let Some(hit) = first.hit(ray, interval) {
                    // Early out: if the hit is before far box's entry, skip far child
                    if hit.t < far_bounds.min {
                        return Some(hit);
                    }
                    // Otherwise, check far child too (with reduced interval)
                    if let Some(hit2) = second.hit(ray, &Interval::new(interval.min, hit.t)) {
                        return Some(if hit.t < hit2.t { hit } else { hit2 });
                    }
                    return Some(hit);
                }

                // No hit in near → must try far
                second.hit(ray, interval)
            }

            (Some(_), None) => self.left.hit(ray, interval),
            (None, Some(_)) => self.right.hit(ray, interval),
            (None, None) => None,
        }
    }
}
