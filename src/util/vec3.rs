use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vec3 {
        let length = self.length_squared().powf(0.5);
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn invert(&self) -> Vec3 {
        Vec3 {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - normal * 2.0 * dot(*self, normal)
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::new(
                rand::random::<f64>() * 2.0 - 1.0,
                rand::random::<f64>() * 2.0 - 1.0,
                rand::random::<f64>() * 2.0 - 1.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, factor: f64) -> Vec3 {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, divisor: f64) -> Vec3 {
        Vec3 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
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

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

pub type Color = Vec3;
