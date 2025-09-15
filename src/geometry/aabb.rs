use crate::{
    geometry::{
        bounds::{Axis, Bounds},
        hittable::Hittable,
        mesh::Mesh,
    },
    util::ray::Ray,
};

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
    pub(crate) fn new(mesh: Mesh<T>) -> Self {
        let bounds = Self::calc_bounds(&mesh.children);

        let num_children = mesh.children.len();
        if num_children < 10 {
            return AABB {
                aabb_type: AABBType::Leaf(mesh.children),
                bounds,
            };
        }

        let longest_axis = bounds.longest_axis();

        let mut sorted_tris = mesh.children;
        sorted_tris.sort_by(|a, b| {
            let a_bounds = a.get_bounds();
            let b_bounds = b.get_bounds();
            let a_center = a_bounds.min.add(a_bounds.max).scale(0.5);
            let b_center = b_bounds.min.add(b_bounds.max).scale(0.5);

            match longest_axis {
                Axis::X => a_center.x.partial_cmp(&b_center.x).unwrap(),
                Axis::Y => a_center.y.partial_cmp(&b_center.y).unwrap(),
                Axis::Z => a_center.z.partial_cmp(&b_center.z).unwrap(),
            }
        });

        let mid = num_children / 2;
        let left_tris = sorted_tris.split_off(mid);
        let right_tris = sorted_tris;

        let left_aabb = AABB::new(Mesh::new(left_tris));
        let right_aabb = AABB::new(Mesh::new(right_tris));
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
    fn hit(&self, ray: &Ray) -> bool {
        match self.bounds.hit(ray) {
            false => false,
            true => match &self.aabb_type {
                AABBType::Recursive(c) => c.hit(ray),
                AABBType::Leaf(children) => children.iter().any(|c| c.hit(ray)),
            },
        }
    }

    fn get_bounds(&self) -> Bounds {
        self.bounds
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

    pub fn hit(&self, ray: &Ray) -> bool {
        self.left.hit(ray) || self.right.hit(ray)
    }
}
