#![allow(clippy::cast_possible_truncation, clippy::many_single_char_names)]

use util::{Point, Unnormalized, Vec3};

use crate::Bounds;

pub fn trs_matrix(
    translation: Option<Vec3>,
    rotation: Option<[f32; 4]>,
    scale: Vec3,
) -> [[f64; 4]; 4] {
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

pub fn mat3_inverse_transpose(m: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
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

pub fn mat4_inverse(m: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
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

pub fn mat4_multiply(a: [[f64; 4]; 4], b: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut r = [[0.0f64; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                r[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    r
}

pub fn mat4_transform_point(m: [[f64; 4]; 4], p: Vec3) -> Vec3 {
    Vec3::new(
        (m[0][0] * f64::from(p.x) + m[0][1] * f64::from(p.y) + m[0][2] * f64::from(p.z) + m[0][3])
            as f32,
        (m[1][0] * f64::from(p.x) + m[1][1] * f64::from(p.y) + m[1][2] * f64::from(p.z) + m[1][3])
            as f32,
        (m[2][0] * f64::from(p.x) + m[2][1] * f64::from(p.y) + m[2][2] * f64::from(p.z) + m[2][3])
            as f32,
    )
}

pub fn mat4_transform_dir<S>(m: [[f64; 4]; 4], d: &Vec3<S>) -> Vec3<Unnormalized> {
    Vec3::new(
        (m[0][0] * f64::from(d.x) + m[0][1] * f64::from(d.y) + m[0][2] * f64::from(d.z)) as f32,
        (m[1][0] * f64::from(d.x) + m[1][1] * f64::from(d.y) + m[1][2] * f64::from(d.z)) as f32,
        (m[2][0] * f64::from(d.x) + m[2][1] * f64::from(d.y) + m[2][2] * f64::from(d.z)) as f32,
    )
}

pub fn transform_bounds_with_matrix(bounds: &Bounds, m: [[f64; 4]; 4]) -> Bounds {
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
        .map(|&c| mat4_transform_point(m, c))
        .collect();

    let min = transformed
        .iter()
        .copied()
        .reduce(|u: util::Vec3, v: util::Vec3| Point::min(&u, &v))
        .unwrap();
    let max = transformed
        .iter()
        .copied()
        .reduce(|u: util::Vec3, v: util::Vec3| Point::max(&u, &v))
        .unwrap();
    Bounds { min, max }
}
