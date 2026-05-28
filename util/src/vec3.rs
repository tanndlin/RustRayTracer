use std::ops;

use rand::{RngExt, SeedableRng, rngs::SmallRng};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len_squared = self.length_squared();
        if len_squared > 1e-8 {
            *self / len_squared.sqrt()
        } else {
            Vec3::zero()
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
        *self - normal * 2.0 * self.dot(normal)
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn random_in_unit_sphere() -> Self {
        THREAD_RNG.with(|thread_rng| {
            let mut thread_rng = thread_rng.borrow_mut();
            loop {
                let p = Vec3::new(
                    thread_rng.random::<f32>() * 2.0 - 1.0,
                    thread_rng.random::<f32>() * 2.0 - 1.0,
                    thread_rng.random::<f32>() * 2.0 - 1.0,
                );
                if p.length_squared() < 1.0 {
                    return p;
                }
            }
        })
    }

    pub fn dot(&self, normal: Vec3) -> f32 {
        self.x * normal.x + self.y * normal.y + self.z * normal.z
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

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
}

thread_local! {
    pub static THREAD_RNG: std::cell::RefCell<SmallRng> =
        std::cell::RefCell::new(SmallRng::from_rng(&mut rand::rng()));
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

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
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

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, factor: f32) -> Vec3 {
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

impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, divisor: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / divisor.x,
            y: self.y / divisor.y,
            z: self.z / divisor.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, divisor: f32) -> Vec3 {
        Vec3 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
        }
    }
}

impl From<&[f64]> for Vec3 {
    fn from(value: &[f64]) -> Self {
        assert!(
            value.len() == 3,
            "{}",
            format!("Expected a 3D vector. Got: {value:?}")
        );
        Self {
            x: value[0] as f32,
            y: value[1] as f32,
            z: value[2] as f32,
        }
    }
}

impl From<&[f64; 3]> for Vec3 {
    fn from(value: &[f64; 3]) -> Self {
        Self {
            x: value[0] as f32,
            y: value[1] as f32,
            z: value[2] as f32,
        }
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(value: [f64; 3]) -> Self {
        Self {
            x: value[0] as f32,
            y: value[1] as f32,
            z: value[2] as f32,
        }
    }
}

impl From<&[f64; 2]> for Vec3 {
    fn from(value: &[f64; 2]) -> Self {
        Self {
            x: value[0] as f32,
            y: value[1] as f32,
            z: 0.0,
        }
    }
}

impl From<[f64; 2]> for Vec3 {
    fn from(value: [f64; 2]) -> Self {
        Self {
            x: value[0] as f32,
            y: value[1] as f32,
            z: 0.0,
        }
    }
}

impl From<Vec<f64>> for Vec3 {
    fn from(value: Vec<f64>) -> Self {
        value.as_slice().into()
    }
}

impl From<f64> for Vec3 {
    fn from(value: f64) -> Self {
        Self {
            x: value as f32,
            y: value as f32,
            z: value as f32,
        }
    }
}

pub type Color = Vec3;
