#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub(crate) fn sub(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub(crate) fn normalize(&self) -> Vec3 {
        let length = self.length_squared().powf(0.5);
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub(crate) fn scale(&self, factor: f64) -> Vec3 {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub(crate) fn negate(&self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub(crate) fn add(&self, scale: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + scale.x,
            y: self.y + scale.y,
            z: self.z + scale.z,
        }
    }

    pub(crate) fn mul(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub(crate) fn invert(&self) -> Vec3 {
        Vec3 {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }
}

pub fn min(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.x.min(v.x),
        y: u.y.min(v.y),
        z: u.z.min(v.z),
    }
}

pub fn max(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.x.max(v.x),
        y: u.y.max(v.y),
        z: u.z.max(v.z),
    }
}
