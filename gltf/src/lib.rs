mod accessor;
mod gltf;

pub use accessor::AccessorData;
pub use gltf::{
    GltfData, Material, Mesh as GltfMesh, MimeType, Node, Primitive, Texture as GltfTexture,
};
