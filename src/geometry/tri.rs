
use crate::{
    geometry::{bounds::Bounds, hittable::Hittable},
    util::{
        hit_result::HitResult,
        interval::Interval,
        ray::Ray,
        vec3::{Vec3, cross, max, min},
    },
};

pub struct Tri {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,

    pub normals: Option<(Vec3, Vec3, Vec3)>,
    pub uvs: Option<(Vec3, Vec3, Vec3)>,
    face_normal: Vec3,
    edge_ab: Vec3,
    edge_ac: Vec3,
    bounds: Bounds,

    material_index: usize,
}

impl Tri {
    pub fn new(
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        normals: Option<(Vec3, Vec3, Vec3)>,
        uvs: Option<(Vec3, Vec3, Vec3)>,

        material_index: usize,
    ) -> Self {
        let edge_ab = v1 - v0;
        let edge_ac = v2 - v0;
        let face_normal = cross(edge_ab, edge_ac);
        let bounds = Bounds {
            min: min(v0, min(v1, v2)) - Vec3::new(1e-6, 1e-6, 1e-6),
            max: max(v0, max(v1, v2)) + Vec3::new(1e-6, 1e-6, 1e-6),
        };

        Tri {
            v0,
            v1,
            v2,
            normals,
            uvs,
            face_normal,
            edge_ab,
            edge_ac,
            bounds,
            material_index,
        }
    }
}

impl Hittable for Tri {
    fn hit(&self, r: &Ray, interval: &Interval) -> Option<HitResult> {
        let ao = r.origin - self.v0;
        let dao = cross(ao, r.dir);

        // Backface culling
        let determinant = -r.dir.dot(self.face_normal);
        if determinant < 1e-6 {
            return None;
        }

        let inv_det = 1.0 / determinant;

        // Calculate dst to triangle
        let dst = ao.dot(self.face_normal) * inv_det;
        if !interval.contains(dst) {
            return None;
        }

        let u = self.edge_ac.dot(dao) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let v = -self.edge_ab.dot(dao) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let interpolated_normal = match self.normals {
            Some((n0, n1, n2)) => {
                let w = 1.0 - u - v;
                let normal = n0 * w + n1 * u + n2 * v;
                normal.normalize()
            }
            None => self.face_normal.normalize(),
        };

        let mut n = interpolated_normal.normalize();
        if !n.is_finite() || n.length_squared() < 1e-6 {
            n = self.face_normal.normalize(); // fallback
        }

        let point = r.at(dst);
        let (u, v) = if let Some((uv0, uv1, uv2)) = self.uvs {
            let w = 1.0 - u - v;
            let uv = uv0 * w + uv1 * u + uv2 * v;
            (uv.x, uv.y)
        } else {
            (0.0, 0.0)
        };

        Some(HitResult {
            normal: n,
            t: dst,
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
        self.v0 = self.v0 + *vec;
        self.v1 = self.v1 + *vec;
        self.v2 = self.v2 + *vec;
    }
}
