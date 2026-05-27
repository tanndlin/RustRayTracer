use crate::util::vec3::Color;

#[derive(Debug)]
pub struct Texture {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}
