use crate::{Normalized, vec3::Vec3};

pub struct HitResult {
    pub normal: Vec3<Normalized>,
    pub tangent: Option<(Vec3<Normalized>, Vec3<Normalized>)>, // Tangent, and Bitangent
    pub t: f32,
    pub point: Vec3,
    pub material_index: Option<usize>,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}
