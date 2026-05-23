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
