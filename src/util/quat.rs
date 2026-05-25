use crate::util::vec3::Vec3;

pub fn quat_inverse(q: [f32; 4]) -> [f32; 4] {
    // for unit quaternions, inverse == conjugate
    [-q[0], -q[1], -q[2], q[3]]
}

pub fn quat_rotate(q: [f32; 4], v: Vec3) -> Vec3 {
    let [qx, qy, qz, qw] = q;
    // sandwich product: q * v * q^-1
    let ix = qw * v.x + qy * v.z - qz * v.y;
    let iy = qw * v.y + qz * v.x - qx * v.z;
    let iz = qw * v.z + qx * v.y - qy * v.x;
    let iw = -qx * v.x - qy * v.y - qz * v.z;

    Vec3 {
        x: ix * qw + iw * -qx + iy * -qz - iz * -qy,
        y: iy * qw + iw * -qy + iz * -qx - ix * -qz,
        z: iz * qw + iw * -qz + ix * -qy - iy * -qx,
    }
}

pub fn from_axis_angle(axis: Vec3, angle_rad: f32) -> [f32; 4] {
    let half_angle = angle_rad * 0.5;
    let s = half_angle.sin();
    [axis.x * s, axis.y * s, axis.z * s, half_angle.cos()]
}

pub fn quat_multiply(q1: [f32; 4], q2: [f32; 4]) -> [f32; 4] {
    let (x1, y1, z1, w1) = (q1[0], q1[1], q1[2], q1[3]);
    let (x2, y2, z2, w2) = (q2[0], q2[1], q2[2], q2[3]);

    [
        w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
        w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2,
        w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2,
        w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
    ]
}
