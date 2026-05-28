#![allow(clippy::similar_names, clippy::many_single_char_names)]

use util::{
    HitResult, Interval, Ray, Vec3,
    quat::{self, quat_rotate},
};

use crate::{bounds::Bounds, hittable::Hittable};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Tri {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,

    pub normals: Option<(Vec3, Vec3, Vec3)>,
    pub uvs: Option<(Vec3, Vec3, Vec3)>,
    tangents: Option<[[f32; 4]; 3]>,
    face_normal: Vec3,
    edge_ab: Vec3,
    edge_ac: Vec3,
    bounds: Bounds,

    material_index: Option<usize>,
}

impl Tri {
    pub fn new(
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        normals: Option<(Vec3, Vec3, Vec3)>,
        uvs: Option<(Vec3, Vec3, Vec3)>,
        tangents: Option<[[f32; 4]; 3]>,
        material_index: Option<usize>,
    ) -> Self {
        let edge_ab = v1 - v0;
        let edge_ac = v2 - v0;
        let face_normal = Vec3::cross(edge_ab, edge_ac);
        let bounds = Bounds {
            min: Vec3::min(v0, Vec3::min(v1, v2)) - Vec3::new(1e-6, 1e-6, 1e-6),
            max: Vec3::max(v0, Vec3::max(v1, v2)) + Vec3::new(1e-6, 1e-6, 1e-6),
        };

        let tangents = if tangents.is_none()
            && let Some(uvs) = uvs
        {
            // Tangent is same for all vertices since its a flat face
            let t = compute_tangent(v0, v1, v2, uvs.0, uvs.1, uvs.2);
            Some([t, t, t])
        } else {
            None
        };

        Tri {
            v0,
            v1,
            v2,
            normals,
            uvs,
            tangents,
            face_normal,
            edge_ab,
            edge_ac,
            bounds,
            material_index,
        }
    }

    fn recompute_derived(&mut self) {
        self.edge_ab = self.v1 - self.v0;
        self.edge_ac = self.v2 - self.v0;
        self.face_normal = Vec3::cross(self.edge_ab, self.edge_ac);
        self.bounds = Bounds {
            min: Vec3::min(self.v0, Vec3::min(self.v1, self.v2)) - Vec3::new(1e-6, 1e-6, 1e-6),
            max: Vec3::max(self.v0, Vec3::max(self.v1, self.v2)) + Vec3::new(1e-6, 1e-6, 1e-6),
        };
    }
}

impl Hittable for Tri {
    fn hit(&self, r: &Ray, interval: &Interval) -> Option<HitResult> {
        let ao = r.origin - self.v0;
        let dao = Vec3::cross(ao, r.dir);

        // Backface culling
        let determinant = -r.dir.dot(self.face_normal);
        // // TODO: Respect double_sided on the material
        if determinant.abs() < 1e-6 {
            return None;
        }

        let is_frontface = determinant > 0.0;

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

        if determinant < 0.0 {
            n = -n;
        }

        let point = r.at(dst);
        let (u, v) = if let Some((uv0, uv1, uv2)) = self.uvs {
            let w = 1.0 - u - v;
            let uv = uv0 * w + uv1 * u + uv2 * v;
            (uv.x, uv.y)
        } else {
            (0.0, 0.0)
        };

        let tangent = if let Some(tangents) = self.tangents {
            let t0 = tangents[0];
            let t1 = tangents[1];
            let t2 = tangents[2];

            let w = 1.0 - u - v;
            let t = Vec3::new(
                t0[0] * w + t1[0] * u + t2[0] * v,
                t0[1] * w + t1[1] * u + t2[1] * v,
                t0[2] * w + t1[2] * u + t2[2] * v,
            )
            .normalize();
            let handedness = t0[3]; // W should be constant across the triangle
            let bitangent = Vec3::cross(n, t) * handedness;
            Some((t, bitangent))
        } else {
            None
        };

        Some(HitResult {
            normal: n,
            tangent,
            t: dst,
            point,
            u,
            v,
            material_index: self.material_index,
            front_face: is_frontface,
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

    fn scale(&mut self, s: &Vec3) {
        self.v0 = self.v0 * *s;
        self.v1 = self.v1 * *s;
        self.v2 = self.v2 * *s;

        self.recompute_derived();
    }

    fn rotate(&mut self, axis: &Vec3, angle_rad: f32) {
        let quat = quat::from_axis_angle(*axis, angle_rad);
        self.v0 = quat_rotate(quat, self.v0);
        self.v1 = quat_rotate(quat, self.v1);
        self.v2 = quat_rotate(quat, self.v2);

        self.recompute_derived();
    }
}

fn compute_tangent(v0: Vec3, v1: Vec3, v2: Vec3, uv0: Vec3, uv1: Vec3, uv2: Vec3) -> [f32; 4] {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let duv1 = uv1 - uv0;
    let duv2 = uv2 - uv0;

    let f = 1.0 / (duv1.x * duv2.y - duv2.x * duv1.y);

    let tangent = Vec3::new(
        f * (duv2.y * edge1.x - duv1.y * edge2.x),
        f * (duv2.y * edge1.y - duv1.y * edge2.y),
        f * (duv2.y * edge1.z - duv1.y * edge2.z),
    )
    .normalize();

    [tangent.x, tangent.y, tangent.z, 1.0] // handedness 1.0, compute properly if needed
}
