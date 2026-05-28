use crate::vec3::Vec3;

pub struct HitResult {
    pub normal: Vec3,
    pub tangent: Option<(Vec3, Vec3)>, // Tangent, and Bitangent
    pub t: f32,
    pub point: Vec3,
    pub material_index: Option<usize>,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}
