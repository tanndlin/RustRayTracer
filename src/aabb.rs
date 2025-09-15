use crate::{hittable::Hittable, mesh::Mesh, ray::Ray, tri::Tri, vec3::Vec3};

pub enum AABBType {
    Recursive(RecursiveAABB),
    Leaf(Vec<Box<dyn Hittable>>),
}

#[allow(clippy::upper_case_acronyms)]
pub struct AABB {
    pub aabb_type: AABBType,

    pub a: Vec3,
    pub b: Vec3,
}

impl AABB {
    pub(crate) fn new(mesh: Mesh) -> Self {
        let (a, b) = Self::calc_bounds(&mesh.tris);

        let num_children = mesh.tris.len();
        if num_children < 10 {
            return AABB {
                aabb_type: AABBType::Leaf(
                    mesh.tris
                        .into_iter()
                        .map(|t| Box::new(t) as Box<dyn Hittable>)
                        .collect(),
                ),
                a,
                b,
            };
        }

        let extent = b.sub(a);
        let axis = if extent.x >= extent.y && extent.x >= extent.z {
            0
        } else if extent.y >= extent.x && extent.y >= extent.z {
            1
        } else {
            2
        };

        let mut sorted_tris = mesh.tris;
        sorted_tris.sort_by(|t1, t2| {
            let centroid1 = (t1.v0.x + t1.v1.x + t1.v2.x) / 3.0;
            let centroid2 = (t2.v0.x + t2.v1.x + t2.v2.x) / 3.0;
            match axis {
                0 => centroid1.partial_cmp(&centroid2).unwrap(),
                1 => {
                    let centroid1 = (t1.v0.y + t1.v1.y + t1.v2.y) / 3.0;
                    let centroid2 = (t2.v0.y + t2.v1.y + t2.v2.y) / 3.0;
                    centroid1.partial_cmp(&centroid2).unwrap()
                }
                _ => {
                    let centroid1 = (t1.v0.z + t1.v1.z + t1.v2.z) / 3.0;
                    let centroid2 = (t2.v0.z + t2.v1.z + t2.v2.z) / 3.0;
                    centroid1.partial_cmp(&centroid2).unwrap()
                }
            }
        });

        let mid = num_children / 2;
        let left_tris = sorted_tris[..mid].to_vec();
        let right_tris = sorted_tris[mid..].to_vec();

        let left_aabb = AABB::new(Mesh::new(left_tris));
        let right_aabb = AABB::new(Mesh::new(right_tris));
        AABB {
            aabb_type: AABBType::Recursive(RecursiveAABB::new(
                Box::new(left_aabb),
                Box::new(right_aabb),
            )),
            a,
            b,
        }
    }

    fn slab_method(&self, ray: &Ray) -> bool {
        let t0s = self.a.sub(ray.origin).mul(ray.inv_dir);
        let t1s = self.b.sub(ray.origin).mul(ray.inv_dir);

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

        tmax >= tmin.max(0.0)
    }

    fn calc_bounds(tris: &Vec<Tri>) -> (Vec3, Vec3) {
        let mut min = Vec3 {
            x: f64::INFINITY,
            y: f64::INFINITY,
            z: f64::INFINITY,
        };
        let mut max = Vec3 {
            x: f64::NEG_INFINITY,
            y: f64::NEG_INFINITY,
            z: f64::NEG_INFINITY,
        };

        for tri in tris {
            for v in [&tri.v0, &tri.v1, &tri.v2] {
                if v.x < min.x {
                    min.x = v.x;
                }
                if v.y < min.y {
                    min.y = v.y;
                }
                if v.z < min.z {
                    min.z = v.z;
                }

                if v.x > max.x {
                    max.x = v.x;
                }
                if v.y > max.y {
                    max.y = v.y;
                }
                if v.z > max.z {
                    max.z = v.z;
                }
            }
        }

        (min, max)
    }
}

impl Hittable for AABB {
    fn hit(&self, ray: &Ray) -> bool {
        match self.slab_method(ray) {
            false => false,
            true => match &self.aabb_type {
                AABBType::Recursive(c) => c.hit(ray),
                AABBType::Leaf(children) => children.iter().any(|c| c.hit(ray)),
            },
        }
    }
}

pub struct RecursiveAABB {
    pub left: Box<AABB>,
    pub right: Box<AABB>,
}

impl RecursiveAABB {
    pub fn new(left: Box<AABB>, right: Box<AABB>) -> Self {
        Self { left, right }
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        self.left.hit(ray) || self.right.hit(ray)
    }
}
