use std::{io::Read, path::Path};

use geometry::HittableType;
use gltf::{GltfData, MimeType};
use material::{MaterialType, Texture};
use util::Color;

use crate::{
    glb::types::{Chunk, ChunkType, GlbHeader},
    gltf_parser::assemble_scene,
};

pub fn parse_glb(path: &str, mat_offset: usize) -> (Vec<HittableType>, Vec<MaterialType>) {
    let mut buffer = vec![];
    // Print the absolute path of the file being read
    let abs_path = std::fs::canonicalize(path).unwrap_or_else(|_| std::path::PathBuf::from(path));
    println!("Reading GLB file from path: {}", abs_path.display());

    std::fs::File::open(path)
        .expect("Failed to open .glb file")
        .read_to_end(&mut buffer)
        .expect("Failed to read .glb file");

    let GlbHeader { version, length } = GlbHeader::from(&buffer[0..12].try_into().unwrap());
    assert!(version == 2, "Unsupported GLB version: {version}");

    println!("Paring GLB version {version} file {path}. Size: {length} bytes");

    let mut chunks = vec![];
    let mut offset = 12;
    while offset < buffer.len() {
        let chunk = parse_chunk(&buffer, offset);
        offset += 8 + chunk.length as usize;
        chunks.push(chunk);
    }

    // First chunk should be JSON GltfData, second chunk should be binary
    let gltf_data = {
        let json_chunk = chunks
            .first()
            .expect("GLB file must contain at least one chunk");
        let json_str = String::from_utf8_lossy(&json_chunk.data);
        // Write the json to a file for debugging
        std::fs::write(format!("debug_gltf_{mat_offset}.json"), &*json_str)
            .expect("Failed to write debug_gltf.json");
        serde_json::from_str::<GltfData>(&json_str).unwrap_or_else(|_| {
            panic!(
                "{}",
                format!("Failed to parse JSON chunk as GltfData. Got {json_str}").to_string()
            )
        })
    };
    let binary_chunk = chunks
        .iter()
        .find(|chunk| matches!(chunk.r#type, ChunkType::Binary))
        .expect("GLB file must contain a binary chunk");
    let binary_chunk_data = &binary_chunk.data;
    let binary_chunk = vec![binary_chunk_data.as_slice()];

    let base_path = Path::new(path)
        .parent()
        .expect("Failed to get parent directory of .glb file");
    assemble_scene(gltf_data, &binary_chunk, mat_offset, base_path)
}

fn parse_chunk(buffer: &[u8], offset: usize) -> Chunk {
    let length = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
    let chunk_type = u32::from_le_bytes(buffer[offset + 4..offset + 8].try_into().unwrap()).into();
    let data = buffer[offset + 8..offset + 8 + length as usize].to_vec();

    Chunk {
        length,
        r#type: chunk_type,
        data,
    }
}

// TODO: I think this flow is copied somewhere
pub fn load_texture(
    binary: &[&[u8]],
    gltf_data: &GltfData,
    tex_index: usize,
    base_path: &Path,
) -> Texture {
    let texture = gltf_data.textures.get(tex_index).unwrap();
    let image = gltf_data.images.get(texture.source).unwrap();

    let (data, mime_type) = if let Some(uri) = &image.uri {
        let texture_path = base_path.join(uri);
        let data = std::fs::read(&texture_path).expect("Failed to read texture file");
        // Get the mime type from the file extension
        let mime_type = match texture_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(str::to_lowercase)
        {
            Some(ref ext) if ext == "png" => MimeType::ImagePng,
            Some(ref ext) if ext == "jpg" || ext == "jpeg" => MimeType::ImageJpeg,
            other => panic!("Unknown texture file extension: {other:?}"),
        };

        (data, mime_type)
    } else if let Some(buffer_view_index) = image.buffer_view {
        // If the image is stored in a buffer view, load it from the binary chunk
        let buffer_view = gltf_data.buffer_views.get(buffer_view_index).unwrap();
        let byte_offset = buffer_view.byte_offset.unwrap_or(0);
        let data =
            binary[buffer_view.buffer][byte_offset..byte_offset + buffer_view.byte_length].to_vec();
        // Clone the mime type to return an owned MimeType (not a reference)
        let mime_type = image.mime_type.expect("Image mime type is missing");
        (data, mime_type)
    } else {
        panic!("Image must have either a URI or a buffer view");
    };

    let image = match mime_type {
        MimeType::ImagePng => image::load_from_memory_with_format(&data, image::ImageFormat::Png)
            .expect("Failed to load PNG texture")
            .to_rgba8(),
        MimeType::ImageJpeg => image::load_from_memory_with_format(&data, image::ImageFormat::Jpeg)
            .expect("Failed to load JPEG texture")
            .to_rgba8(),
    };

    let width = image.width() as usize;
    let height = image.height() as usize;
    let pixels = image
        .into_raw()
        .chunks(4)
        .map(|rgba| {
            Color::new(
                f32::from(rgba[0]) / 255.0,
                f32::from(rgba[1]) / 255.0,
                f32::from(rgba[2]) / 255.0,
            )
        })
        .collect();

    Texture {
        data: pixels,
        width,
        height,
    }
}
