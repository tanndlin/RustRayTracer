use crate::util::parser::glb::gltf::{Accessor, AccessorType, ComponentType, GltfData};

impl Accessor {
    pub fn get_data(&self, gltf_data: &GltfData, binary: &[u8]) -> AccessorData {
        let buffer_view = &gltf_data.buffer_views[self.buffer_view as usize];

        let byte_offset = buffer_view.byte_offset + self.byte_offset.unwrap_or(0);
        let byte_length =
            self.count * self.accessor_type.component_count() * component_size(self.component_type);

        let data = &binary[byte_offset as usize..(byte_offset + byte_length) as usize];

        AccessorData::from((&self.accessor_type, self.component_type, data))
    }
}

impl AccessorType {
    fn component_count(&self) -> i64 {
        match self {
            AccessorType::Scalar => 1,
            AccessorType::Vec2 => 2,
            AccessorType::Vec3 => 3,
            AccessorType::Vec4 => 4,
            AccessorType::Mat2 => 4,
            AccessorType::Mat3 => 9,
            AccessorType::Mat4 => 16,
        }
    }
}

#[derive(Debug)]
pub enum AccessorData {
    Scalar(Vec<f64>),
    Vec2(Vec<[f64; 2]>),
    Vec3(Vec<[f64; 3]>),
    Vec4(Vec<[f64; 4]>),
    Mat2(Vec<[[f64; 2]; 2]>),
    Mat3(Vec<[[f64; 3]; 3]>),
    Mat4(Vec<[[f64; 4]; 4]>),
}

impl From<(&AccessorType, ComponentType, &[u8])> for AccessorData {
    fn from((accessor_type, component_type, data): (&AccessorType, ComponentType, &[u8])) -> Self {
        match accessor_type {
            AccessorType::Scalar => AccessorData::into_scalar(component_type, data),
            AccessorType::Vec2 => AccessorData::into_vec2(component_type, data),
            AccessorType::Vec3 => AccessorData::into_vec3(component_type, data),
            AccessorType::Vec4 => AccessorData::into_vec4(component_type, data),
            AccessorType::Mat2 => AccessorData::into_mat2(component_type, data),
            AccessorType::Mat3 => AccessorData::into_mat3(component_type, data),
            AccessorType::Mat4 => AccessorData::into_mat4(component_type, data),
        }
    }
}

impl AccessorData {
    fn into_scalar(component_type: ComponentType, data: &[u8]) -> Self {
        let cs = component_size(component_type) as usize;
        AccessorData::Scalar(
            data.chunks(cs)
                .map(|chunk| read_component(component_type, chunk))
                .collect(),
        )
    }

    fn into_vec2(component_type: ComponentType, data: &[u8]) -> Self {
        let cs = component_size(component_type) as usize;
        AccessorData::Vec2(
            data.chunks(cs * 2)
                .map(|chunk| read_components(component_type, cs, chunk))
                .collect(),
        )
    }

    fn into_vec3(component_type: ComponentType, data: &[u8]) -> Self {
        let cs = component_size(component_type) as usize;
        AccessorData::Vec3(
            data.chunks(cs * 3)
                .map(|chunk| read_components(component_type, cs, chunk))
                .collect(),
        )
    }

    fn into_vec4(component_type: ComponentType, data: &[u8]) -> Self {
        let cs = component_size(component_type) as usize;
        AccessorData::Vec4(
            data.chunks(cs * 4)
                .map(|chunk| read_components(component_type, cs, chunk))
                .collect(),
        )
    }

    fn into_mat2(component_type: ComponentType, data: &[u8]) -> Self {
        let cs = component_size(component_type) as usize;
        AccessorData::Mat2(
            data.chunks(cs * 4)
                .map(|chunk| {
                    std::array::from_fn(|r| {
                        read_components(component_type, cs, &chunk[r * cs * 2..(r + 1) * cs * 2])
                    })
                })
                .collect(),
        )
    }

    fn into_mat3(component_type: ComponentType, data: &[u8]) -> Self {
        let cs = component_size(component_type) as usize;
        AccessorData::Mat3(
            data.chunks(cs * 9)
                .map(|chunk| {
                    std::array::from_fn(|r| {
                        read_components(component_type, cs, &chunk[r * cs * 3..(r + 1) * cs * 3])
                    })
                })
                .collect(),
        )
    }

    fn into_mat4(component_type: ComponentType, data: &[u8]) -> Self {
        let cs = component_size(component_type) as usize;
        AccessorData::Mat4(
            data.chunks(cs * 16)
                .map(|chunk| {
                    std::array::from_fn(|r| {
                        read_components(component_type, cs, &chunk[r * cs * 4..(r + 1) * cs * 4])
                    })
                })
                .collect(),
        )
    }
}

impl AccessorData {
    pub fn len(&self) -> usize {
        match self {
            AccessorData::Scalar(items) => items.len(),
            AccessorData::Vec2(items) => items.len(),
            AccessorData::Vec3(items) => items.len(),
            AccessorData::Vec4(items) => items.len(),
            AccessorData::Mat2(items) => items.len(),
            AccessorData::Mat3(items) => items.len(),
            AccessorData::Mat4(items) => items.len(),
        }
    }
}

fn component_size(ct: ComponentType) -> i64 {
    match ct {
        ComponentType::Byte | ComponentType::UnsignedByte => 1,
        ComponentType::Short | ComponentType::UnsignedShort => 2,
        ComponentType::UnsignedInt | ComponentType::Float => 4,
    }
}

fn read_component(component_type: ComponentType, chunk: &[u8]) -> f64 {
    match component_type {
        ComponentType::Byte => i8::from_le_bytes(chunk.try_into().unwrap()) as f64,
        ComponentType::UnsignedByte => u8::from_le_bytes(chunk.try_into().unwrap()) as f64,
        ComponentType::Short => i16::from_le_bytes(chunk.try_into().unwrap()) as f64,
        ComponentType::UnsignedShort => u16::from_le_bytes(chunk.try_into().unwrap()) as f64,
        ComponentType::UnsignedInt => u32::from_le_bytes(chunk.try_into().unwrap()) as f64,
        ComponentType::Float => f32::from_le_bytes(chunk.try_into().unwrap()) as f64,
    }
}

fn read_components<const N: usize>(
    component_type: ComponentType,
    cs: usize,
    chunk: &[u8],
) -> [f64; N] {
    std::array::from_fn(|i| read_component(component_type, &chunk[i * cs..(i + 1) * cs]))
}
