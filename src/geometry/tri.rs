use crate::{
    geometry::{bounds::Bounds, hittable::Hittable},
    util::{
        hit_result::HitResult,
        interval::Interval,
        ray::Ray,
        vec3::{Vec3, cross, dot, max, min},
    },
};

#[derive(Clone, Copy, Debug)]
pub struct Tri {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,

    pub n0: Option<Vec3>,
    pub n1: Option<Vec3>,
    pub n2: Option<Vec3>,

    face_normal: Vec3,
    edge_ab: Vec3,
    edge_ac: Vec3,

    material_index: usize,
}

impl Tri {
    pub fn new(
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        n0: Option<Vec3>,
        n1: Option<Vec3>,
        n2: Option<Vec3>,
    ) -> Self {
        let edge_ab = v1 - v0;
        let edge_ac = v2 - v0;
        let face_normal = cross(edge_ab, edge_ac);

        Tri {
            v0,
            v1,
            v2,
            n0,
            n1,
            n2,
            face_normal,
            edge_ab,
            edge_ac,
            material_index: 0,
        }
    }
}

impl Hittable for Tri {
    fn hit(&self, r: &Ray, interval: &Interval) -> Option<HitResult> {
        let ao = r.origin - self.v0;
        let dao = cross(ao, r.dir);

        // Backface culling
        let determinant = -dot(r.dir, self.face_normal);
        if determinant < 1e-6 {
            return None;
        }

        let inv_det = 1.0 / determinant;

        // Calculate dst to triangle
        let dst = dot(ao, self.face_normal) * inv_det;
        if dst < 0.0 || !interval.contains(dst) {
            return None;
        }

        let u = dot(self.edge_ac, dao) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let v = -dot(self.edge_ab, dao) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let interpolated_normal =
            if let (Some(n0), Some(n1), Some(n2)) = (self.n0, self.n1, self.n2) {
                let w = 1.0 - u - v;
                let normal = n0 * w + n1 * u + n2 * v;
                normal.normalize()
            } else {
                self.face_normal.normalize()
            };

        let mut n = interpolated_normal.normalize();
        if !n.is_finite() || n.length_squared() < 1e-6 {
            n = self.face_normal.normalize(); // fallback
        }

        let point = r.at(dst);
        Some(HitResult {
            normal: n,
            t: dst,
            point,
            material_index: self.material_index,
        })
    }

    fn get_bounds(&self) -> Bounds {
        let min = min(self.v0, min(self.v1, self.v2))
            - Vec3 {
                x: 1e-6,
                y: 1e-6,
                z: 1e-6,
            };
        let max = max(self.v0, max(self.v1, self.v2))
            + Vec3 {
                x: 1e-6,
                y: 1e-6,
                z: 1e-6,
            };

        Bounds { min, max }
    }
}
