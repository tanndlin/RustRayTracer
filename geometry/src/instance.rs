#![allow(clippy::cast_possible_truncation, clippy::many_single_char_names)]
use std::sync::Arc;

use gltf::gltf::Node;
use util::{
    hit_result::HitResult,
    interval::Interval,
    quat::{from_axis_angle, quat_multiply, quat_rotate},
    ray::Ray,
    vec3::{Vec3, max, min},
};

use crate::{
    bounds::Bounds,
    hittable::{Hittable, HittableType},
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Instance {
    pub name: String,
    pub translation: Option<Vec3>,
    pub rotation: Option<[f32; 4]>,
    pub scale: Vec3,
    pub world_to_object: [[f64; 4]; 4], // inverse TRS
    pub object_to_world: [[f64; 4]; 4], // TRS
    normal_matrix: [[f64; 4]; 4],
    bounds: Bounds, // World Space
    pub base: Arc<HittableType>,
}

impl Instance {
    pub fn new(
        name: String,
        translation: Option<Vec3>,
        rotation: Option<[f32; 4]>,
        scale: Option<Vec3>,
        base: Arc<HittableType>,
    ) -> Self {
        let scale = scale.unwrap_or(Vec3::from(1.0));
        let object_to_world = trs_matrix(translation, rotation, scale);
        let world_to_object = mat4_inverse(object_to_world);
        let normal_matrix = mat3_inverse_transpose(object_to_world);

        Self {
            name,
            translation,
            rotation,
            scale,
            object_to_world,
            world_to_object,
            normal_matrix,
            bounds: Self::calc_bounds(&base, translation, rotation, scale),
            base,
        }
    }
}

impl Hittable for Instance {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        self.get_bounds().hit(ray, interval)?;

        let origin = mat4_transform_point(self.world_to_object, ray.origin);
        let dir_transformed = mat4_transform_dir(self.world_to_object, ray.dir);
        let dir_length = dir_transformed.length();
        let dir = dir_transformed / dir_length;

        let transformed_ray = Ray::new(origin, dir);
        let transformed_interval = Interval {
            min: interval.min * dir_length,
            max: interval.max * dir_length,
        };

        let mut hit = self.base.hit(&transformed_ray, &transformed_interval)?;

        // t is in object space with normalized dir, scale back to world space
        hit.t /= dir_length;

        hit.point = mat4_transform_point(self.object_to_world, hit.point);
        hit.normal = mat4_transform_dir(self.normal_matrix, hit.normal).normalize();
        Some(hit)
    }

    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn translate(&mut self, vec: &Vec3) {
        match self.translation {
            Some(t) => self.translation = Some(t + vec),
            None => self.translation = Some(*vec),
        }

        self.recompute_bounds();
    }

    fn scale(&mut self, vec: &Vec3) {
        self.scale = self.scale * *vec;
        self.recompute_bounds();
    }

    fn rotate(&mut self, axis: &Vec3, angle_rad: f32) {
        let rot = from_axis_angle(*axis, angle_rad);
        self.rotation = match self.rotation {
            Some(r) => Some(quat_multiply(rot, r)),
            None => Some(rot),
        };

        self.recompute_bounds();
    }
}

impl From<(&[Arc<HittableType>], Node)> for Instance {
    fn from(value: (&[Arc<HittableType>], Node)) -> Self {
        let (meshes, node) = value;
        let mesh_index = node
            .mesh
            .expect("GLTF node must have a mesh to be instanced");

        let translation = node.translation.map(Vec3::from);
        let rotation = node.rotation.map(|r| {
            let arr: [f64; 4] = r.try_into().unwrap();
            arr.map(|f| f as f32)
        });
        let scale = node.scale.map(Vec3::from);

        let base = meshes
            .get(mesh_index)
            .expect("Mesh index out of bounds for GLTF node")
            .clone();

        Self::new(node.name, translation, rotation, scale, base)
    }
}

impl Instance {
    #[allow(dead_code)]
    fn recompute_bounds(&mut self) {
        self.bounds = Self::calc_bounds(&self.base, self.translation, self.rotation, self.scale);

        self.object_to_world = trs_matrix(self.translation, self.rotation, self.scale);
        self.world_to_object = mat4_inverse(self.object_to_world);
        self.normal_matrix = mat3_inverse_transpose(self.object_to_world);
    }

    fn calc_bounds(
        base: &Arc<HittableType>,
        translation: Option<Vec3>,
        rotation: Option<[f32; 4]>,
        scale: Vec3,
    ) -> Bounds {
        let bounds = base.get_bounds();

        let corners = [
            Vec3::new(bounds.min.x, bounds.min.y, bounds.min.z),
            Vec3::new(bounds.max.x, bounds.min.y, bounds.min.z),
            Vec3::new(bounds.min.x, bounds.max.y, bounds.min.z),
            Vec3::new(bounds.min.x, bounds.min.y, bounds.max.z),
            Vec3::new(bounds.max.x, bounds.max.y, bounds.min.z),
            Vec3::new(bounds.max.x, bounds.min.y, bounds.max.z),
            Vec3::new(bounds.min.x, bounds.max.y, bounds.max.z),
            Vec3::new(bounds.max.x, bounds.max.y, bounds.max.z),
        ];

        let transformed: Vec<Vec3> = corners
            .iter()
            .map(|&c| {
                let mut p = c;
                p = p * scale;
                if let Some(q) = rotation {
                    p = quat_rotate(q, p);
                }
                if let Some(t) = translation {
                    p = p + t;
                }
                p
            })
            .collect();

        let min = transformed.iter().copied().reduce(min).unwrap();
        let max = transformed.iter().copied().reduce(max).unwrap();
        Bounds { min, max }
    }
}

fn trs_matrix(translation: Option<Vec3>, rotation: Option<[f32; 4]>, scale: Vec3) -> [[f64; 4]; 4] {
    // Start with identity
    let mut m = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    // Scale
    m[0][0] = f64::from(scale.x);
    m[1][1] = f64::from(scale.y);
    m[2][2] = f64::from(scale.z);

    // Rotation (quaternion to matrix, applied after scale)
    if let Some([qx, qy, qz, qw]) = rotation {
        let (qx, qy, qz, qw) = (f64::from(qx), f64::from(qy), f64::from(qz), f64::from(qw));
        let r = [
            [
                1.0 - 2.0 * (qy * qy + qz * qz),
                2.0 * (qx * qy - qz * qw),
                2.0 * (qx * qz + qy * qw),
            ],
            [
                2.0 * (qx * qy + qz * qw),
                1.0 - 2.0 * (qx * qx + qz * qz),
                2.0 * (qy * qz - qx * qw),
            ],
            [
                2.0 * (qx * qz - qy * qw),
                2.0 * (qy * qz + qx * qw),
                1.0 - 2.0 * (qx * qx + qy * qy),
            ],
        ];

        // Combine R * S (current m is scale)
        let mut rs = [[0.0f64; 4]; 4];
        for i in 0..3 {
            for j in 0..3 {
                rs[i][j] = r[i][0] * m[0][j] + r[i][1] * m[1][j] + r[i][2] * m[2][j];
            }
        }
        rs[3][3] = 1.0;
        m = rs;
    }

    // Translation (just set the last column)
    if let Some(t) = translation {
        m[0][3] = f64::from(t.x);
        m[1][3] = f64::from(t.y);
        m[2][3] = f64::from(t.z);
    }

    m
}

fn mat3_inverse_transpose(m: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
    // Extract upper 3x3, compute inverse transpose
    let a = m[0][0];
    let b = m[0][1];
    let c = m[0][2];
    let d = m[1][0];
    let e = m[1][1];
    let f = m[1][2];
    let g = m[2][0];
    let h = m[2][1];
    let k = m[2][2];

    let det = a * (e * k - f * h) - b * (d * k - f * g) + c * (d * h - e * g);
    let inv_det = 1.0 / det;

    // Inverse then transpose (or equivalently cofactor matrix / det)
    let mut r = [[0.0f64; 4]; 4];
    r[0][0] = (e * k - f * h) * inv_det;
    r[1][0] = (c * h - b * k) * inv_det;
    r[2][0] = (b * f - c * e) * inv_det;
    r[0][1] = (f * g - d * k) * inv_det;
    r[1][1] = (a * k - c * g) * inv_det;
    r[2][1] = (c * d - a * f) * inv_det;
    r[0][2] = (d * h - e * g) * inv_det;
    r[1][2] = (b * g - a * h) * inv_det;
    r[2][2] = (a * e - b * d) * inv_det;
    r[3][3] = 1.0;
    r
}

fn mat4_inverse(m: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
    // For TRS matrices, inverse is S^-1 * R^T * T^-1
    // Extract and invert each component
    let tx = m[0][3];
    let ty = m[1][3];
    let tz = m[2][3];

    // Upper 3x3 inverse via adjugate (works for RS matrices)
    let a = m[0][0];
    let b = m[0][1];
    let c = m[0][2];
    let d = m[1][0];
    let e = m[1][1];
    let f = m[1][2];
    let g = m[2][0];
    let h = m[2][1];
    let k = m[2][2];

    let det = a * (e * k - f * h) - b * (d * k - f * g) + c * (d * h - e * g);
    let inv_det = 1.0 / det;

    let mut inv = [[0.0f64; 4]; 4];
    inv[0][0] = (e * k - f * h) * inv_det;
    inv[0][1] = (c * h - b * k) * inv_det;
    inv[0][2] = (b * f - c * e) * inv_det;
    inv[1][0] = (f * g - d * k) * inv_det;
    inv[1][1] = (a * k - c * g) * inv_det;
    inv[1][2] = (c * d - a * f) * inv_det;
    inv[2][0] = (d * h - e * g) * inv_det;
    inv[2][1] = (b * g - a * h) * inv_det;
    inv[2][2] = (a * e - b * d) * inv_det;

    // Inverse translation: -R^-1 * t
    inv[0][3] = -(inv[0][0] * tx + inv[0][1] * ty + inv[0][2] * tz);
    inv[1][3] = -(inv[1][0] * tx + inv[1][1] * ty + inv[1][2] * tz);
    inv[2][3] = -(inv[2][0] * tx + inv[2][1] * ty + inv[2][2] * tz);
    inv[3][3] = 1.0;

    inv
}

fn mat4_transform_point(m: [[f64; 4]; 4], p: Vec3) -> Vec3 {
    Vec3::new(
        (m[0][0] * f64::from(p.x) + m[0][1] * f64::from(p.y) + m[0][2] * f64::from(p.z) + m[0][3])
            as f32,
        (m[1][0] * f64::from(p.x) + m[1][1] * f64::from(p.y) + m[1][2] * f64::from(p.z) + m[1][3])
            as f32,
        (m[2][0] * f64::from(p.x) + m[2][1] * f64::from(p.y) + m[2][2] * f64::from(p.z) + m[2][3])
            as f32,
    )
}

fn mat4_transform_dir(m: [[f64; 4]; 4], d: Vec3) -> Vec3 {
    // Directions ignore translation
    Vec3::new(
        (m[0][0] * f64::from(d.x) + m[0][1] * f64::from(d.y) + m[0][2] * f64::from(d.z)) as f32,
        (m[1][0] * f64::from(d.x) + m[1][1] * f64::from(d.y) + m[1][2] * f64::from(d.z)) as f32,
        (m[2][0] * f64::from(d.x) + m[2][1] * f64::from(d.y) + m[2][2] * f64::from(d.z)) as f32,
    )
}
