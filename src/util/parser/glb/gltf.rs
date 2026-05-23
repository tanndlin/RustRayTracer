use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Accessor {
    pub buffer_view: i64,
    pub component_type: ComponentType,
    pub count: i64,
    pub max: Option<Vec<f64>>,
    pub min: Option<Vec<f64>>,
    pub byte_offset: Option<i64>,
    #[serde(rename = "type")]
    pub accessor_type: AccessorType,
}

#[derive(Debug, Clone, Copy)]
pub enum ComponentType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    UnsignedInt,
    Float,
}

impl TryFrom<i64> for ComponentType {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            5120 => Ok(ComponentType::Byte),
            5121 => Ok(ComponentType::UnsignedByte),
            5122 => Ok(ComponentType::Short),
            5123 => Ok(ComponentType::UnsignedShort),
            5125 => Ok(ComponentType::UnsignedInt),
            5126 => Ok(ComponentType::Float),
            _ => Err(format!("Unknown component type: {}", value)),
        }
    }
}

impl<'de> Deserialize<'de> for ComponentType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = i64::deserialize(d)?;
        ComponentType::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AccessorType {
    #[serde(rename = "SCALAR")]
    Scalar,
    #[serde(rename = "VEC2")]
    Vec2,
    #[serde(rename = "VEC3")]
    Vec3,
    #[serde(rename = "VEC4")]
    Vec4,
    #[serde(rename = "MAT2")]
    Mat2,
    #[serde(rename = "MAT3")]
    Mat3,
    #[serde(rename = "MAT4")]
    Mat4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    generator: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    pub buffer: i64,
    pub byte_length: i64,
    pub byte_offset: i64,
    pub target: Option<i64>,
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
    pub primitives: Vec<Primitive>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Primitive {
    pub attributes: Attributes,
    pub indices: usize,
    pub material: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Attributes {
    pub position: usize,
    normal: usize,
    texcoord_0: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub mesh: Option<usize>,
    pub name: String,
    pub rotation: Option<Vec<f64>>,
    pub scale: Option<Vec<f64>>,
    pub translation: Option<Vec<f64>>,
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
    pub nodes: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextureElement {
    sampler: i64,
    source: i64,
}
