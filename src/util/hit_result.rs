use crate::util::vec3::Vec3;

pub struct HitResult {
    pub normal: Vec3,
    pub t: f32,
    pub point: Vec3,
    pub material_index: usize,
    pub u: f32,
    pub v: f32,
}
