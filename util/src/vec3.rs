use rand::{RngExt, SeedableRng, rngs::SmallRng};
use std::marker::PhantomData;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Normalized;
#[derive(Clone, Copy, Debug)]
pub struct Unnormalized;

#[derive(Clone, Copy, Debug)]
pub struct Vec3<State = Unnormalized> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _state: PhantomData<State>,
}

impl<S> Vec3<S> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            _state: PhantomData,
        }
    }

    pub fn zero() -> Vec3<Unnormalized> {
        Vec3::<Unnormalized> {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            _state: PhantomData,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn invert(&self) -> Vec3<Unnormalized> {
        Vec3 {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
            _state: PhantomData,
        }
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn random_in_unit_sphere() -> Vec3<Unnormalized> {
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

    pub fn dot<T>(&self, other: &Vec3<T>) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn min(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: u.x.min(v.x),
            y: u.y.min(v.y),
            z: u.z.min(v.z),
            _state: PhantomData,
        }
    }

    pub fn max(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: u.x.max(v.x),
            y: u.y.max(v.y),
            z: u.z.max(v.z),
            _state: PhantomData,
        }
    }

    pub fn cross<T>(u: &Vec3<S>, v: &Vec3<T>) -> Vec3<Unnormalized> {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
            _state: PhantomData,
        }
    }
}

thread_local! {
    pub static THREAD_RNG: std::cell::RefCell<SmallRng> =
        std::cell::RefCell::new(SmallRng::from_rng(&mut rand::rng()));
}

impl<S, T> ops::Add<Vec3<T>> for Vec3<S> {
    type Output = Vec3<Unnormalized>;

    fn add(self, other: Vec3<T>) -> Vec3<Unnormalized> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            _state: PhantomData,
        }
    }
}

impl<S, T> ops::Add<&Vec3<T>> for Vec3<S> {
    type Output = Vec3<Unnormalized>;

    fn add(self, other: &Vec3<T>) -> Vec3<Unnormalized> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            _state: PhantomData,
        }
    }
}

impl<S, T> ops::Sub<Vec3<T>> for Vec3<S> {
    type Output = Vec3<Unnormalized>;

    fn sub(self, other: Vec3<T>) -> Vec3<Unnormalized> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            _state: PhantomData,
        }
    }
}

impl<S> ops::Mul for Vec3<S> {
    type Output = Vec3<Unnormalized>;

    fn mul(self, other: Vec3<S>) -> Vec3<Unnormalized> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            _state: PhantomData,
        }
    }
}

impl<S> ops::Mul<f32> for Vec3<S> {
    type Output = Vec3<Unnormalized>;

    fn mul(self, factor: f32) -> Vec3<Unnormalized> {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            _state: PhantomData,
        }
    }
}

impl<S> ops::Neg for Vec3<S> {
    type Output = Vec3<S>;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            _state: PhantomData,
        }
    }
}

impl<S> ops::Div for Vec3<S> {
    type Output = Vec3<S>;

    fn div(self, divisor: Vec3<S>) -> Self {
        Self {
            x: self.x / divisor.x,
            y: self.y / divisor.y,
            z: self.z / divisor.z,
            _state: PhantomData,
        }
    }
}

impl<S> ops::Div<f32> for Vec3<S> {
    type Output = Vec3<Unnormalized>;

    fn div(self, divisor: f32) -> Vec3<Unnormalized> {
        Vec3 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
            _state: PhantomData,
        }
    }
}

impl Vec3<Unnormalized> {
    #[must_use]
    pub fn normalize(&self) -> Vec3<Normalized> {
        let len_squared = self.length_squared();
        if len_squared > 1e-8 {
            let len = len_squared.sqrt();
            Vec3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                _state: PhantomData,
            }
        } else {
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                _state: PhantomData,
            }
        }
    }
}

impl Vec3<Normalized> {
    #[must_use]
    pub fn reflect(&self, normal: &Vec3<Normalized>) -> Vec3<Normalized> {
        let dot = self.dot(normal);
        Vec3 {
            x: self.x - normal.x * 2.0 * dot,
            y: self.y - normal.y * 2.0 * dot,
            z: self.z - normal.z * 2.0 * dot,
            _state: PhantomData,
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
            _state: PhantomData,
        }
    }
}

impl From<&[f64; 3]> for Vec3 {
    fn from(value: &[f64; 3]) -> Self {
        Self {
            x: value[0] as f32,
            y: value[1] as f32,
            z: value[2] as f32,
            _state: PhantomData,
        }
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(value: [f64; 3]) -> Self {
        (&value).into()
    }
}

impl From<&[f64; 2]> for Vec3 {
    fn from(value: &[f64; 2]) -> Self {
        Self {
            x: value[0] as f32,
            y: value[1] as f32,
            z: 0.0,
            _state: PhantomData,
        }
    }
}

impl From<[f64; 2]> for Vec3 {
    fn from(value: [f64; 2]) -> Self {
        Self {
            x: value[0] as f32,
            y: value[1] as f32,
            z: 0.0,
            _state: PhantomData,
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
            _state: PhantomData,
        }
    }
}

pub type Color = Vec3<Unnormalized>;
pub type Point = Vec3<Unnormalized>;
