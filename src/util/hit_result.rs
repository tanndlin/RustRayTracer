use crate::util::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitResult {
    pub normal: Vec3,
    pub t: f64,
    pub point: Vec3,
    pub material_index: usize,
    pub u: f64,
    pub v: f64,
}
