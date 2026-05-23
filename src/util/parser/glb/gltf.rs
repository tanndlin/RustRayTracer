use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GltfData {
    pub asset: Asset,
    pub scene: i64,
    pub scenes: Vec<Scene>,
    pub nodes: Vec<Node>,
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh>,
    pub textures: Vec<TextureElement>,
    pub images: Vec<Image>,
    pub accessors: Vec<Accessor>,
    pub buffer_views: Vec<BufferView>,
    pub samplers: Vec<Sampler>,
    pub buffers: Vec<Buffer>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Accessor {
    buffer_view: i64,
    component_type: i64,
    count: i64,
    max: Option<Vec<f64>>,
    min: Option<Vec<f64>>,
    #[serde(rename = "type")]
    accessor_type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    #[serde(rename = "SCALAR")]
    Scalar,
    #[serde(rename = "VEC2")]
    Vec2,
    #[serde(rename = "VEC3")]
    Vec3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    generator: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    buffer: i64,
    byte_length: i64,
    byte_offset: i64,
    target: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    byte_length: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    buffer_view: i64,
    mime_type: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    double_sided: bool,
    name: String,
    normal_texture: Option<Texture>,
    pbr_metallic_roughness: PbrMetallicRoughness,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Texture {
    index: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PbrMetallicRoughness {
    base_color_texture: Option<Texture>,
    metallic_factor: i64,
    metallic_roughness_texture: Option<Texture>,
    base_color_factor: Option<Vec<f64>>,
    roughness_factor: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mesh {
    name: String,
    primitives: Vec<Primitive>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Primitive {
    attributes: Attributes,
    indices: i64,
    material: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Attributes {
    position: i64,
    normal: i64,
    texcoord_0: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    mesh: Option<i64>,
    name: String,
    rotation: Option<Vec<f64>>,
    scale: Option<Vec<f64>>,
    translation: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sampler {
    mag_filter: i64,
    min_filter: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    name: String,
    nodes: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextureElement {
    sampler: i64,

    source: i64,
}
