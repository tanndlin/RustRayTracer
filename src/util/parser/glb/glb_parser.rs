use std::{io::Read, sync::Arc};

use crate::{
    geometry::{hittable::HittableType, instance::Instance},
    material::{lambertian::LambertianBase, material_trait::MaterialType},
    util::{
        parser::glb::{
            gltf::{GltfData, Material, MimeType},
            types::{Chunk, ChunkType},
        },
        vec3::Color,
    },
};

pub fn parse_glb(path: &str) -> (Vec<HittableType>, Vec<MaterialType>) {
    let mut buffer = vec![];
    std::fs::File::open(path)
        .expect("Failed to open .glb file")
        .read_to_end(&mut buffer)
        .expect("Failed to read .glb file");

    let preamble = &buffer[0..12];
    if preamble[0..4] != [0x67, 0x6C, 0x54, 0x46] {
        panic!("Invalid GLB file: missing 'glTF' magic header");
    }

    let version = u32::from_le_bytes(preamble[4..8].try_into().unwrap());
    if version != 2 {
        panic!("Unsupported GLB version: {}", version);
    }

    let length = u32::from_le_bytes(preamble[8..12].try_into().unwrap());
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
        serde_json::from_str::<GltfData>(&json_str).expect("Failed to parse JSON chunk as GltfData")
    };
    let binary_chunk = chunks
        .iter()
        .find(|chunk| matches!(chunk.chunk_type, ChunkType::Binary))
        .expect("GLB file must contain a binary chunk");

    assemble_scene(gltf_data, binary_chunk)
}

fn assemble_scene(
    gltf_data: GltfData,
    binary_chunk: &Chunk,
) -> (Vec<HittableType>, Vec<MaterialType>) {
    let scene = gltf_data
        .scenes
        .get(gltf_data.scene as usize)
        .expect("Scene index out of bounds");

    let instance_bases = gltf_data
        .meshes
        .iter()
        .map(|mesh| {
            Arc::new(HittableType::from_gltf_mesh(
                mesh,
                &gltf_data,
                &binary_chunk.data,
            ))
        })
        .collect::<Vec<_>>();

    println!("Parsed {} meshes", instance_bases.len());

    // Nodes are the instances of the meshes
    let nodes = scene
        .nodes
        .iter()
        .map(|&node_index| {
            gltf_data
                .nodes
                .get(node_index as usize)
                .expect("Node index out of bounds")
        })
        .collect::<Vec<_>>();

    let instances: Vec<HittableType> = nodes
        .iter()
        .map(|node| HittableType::Instance((instance_bases.as_slice(), (*node).clone()).into()))
        .collect();

    let mut materials = vec![];
    for mat in gltf_data.materials {
        let Material {
            name,
            normal_texture,
            pbr_metallic_roughness: pbr,
            ..
        } = mat;

        dbg!(&name);

        let material = match pbr.base_color_texture {
            Some(tex) => {
                println!("Is image");
                let texture = gltf_data.textures.get(tex.index).unwrap();
                let image = gltf_data.images.get(texture.source).unwrap();
                let buffer_view = gltf_data.buffer_views.get(image.buffer_view).unwrap();
                let data = &binary_chunk.data
                    [buffer_view.byte_offset..buffer_view.byte_offset + buffer_view.byte_length];
                let image = match image.mime_type {
                    MimeType::ImagePng => {
                        image::load_from_memory_with_format(data, image::ImageFormat::Png)
                            .expect("Failed to load PNG texture")
                            .to_rgba8()
                    }
                    MimeType::ImageJpeg => {
                        image::load_from_memory_with_format(data, image::ImageFormat::Jpeg)
                            .expect("Failed to load JPEG texture")
                            .to_rgba8()
                    }
                };

                let (width, height) = image.dimensions();
                dbg!(&width, &height);

                let pixels = image
                    .into_raw()
                    .chunks(4)
                    .map(|rgba| {
                        let r = rgba[0] as f32 / 255.0;
                        let g = rgba[1] as f32 / 255.0;
                        let b = rgba[2] as f32 / 255.0;
                        Color::new(r, g, b)
                    })
                    .collect::<Vec<_>>();

                MaterialType::TextureLambertian(LambertianBase {
                    name,
                    albedo: pixels,
                    roughness: 1.0,
                    alpha: 1.0, // TODO Support alpha channel
                })
            }
            None => {
                println!("Is albedo");
                let rgba = pbr.base_color_factor.unwrap();
                MaterialType::Lambertian(LambertianBase {
                    name,
                    albedo: rgba[..3].into(),
                    roughness: 1.0,
                    alpha: rgba[3] as f32,
                })
            }
        };

        materials.push(material);
    }

    (instances, materials)
}

fn parse_chunk(buffer: &[u8], offset: usize) -> Chunk {
    let length = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
    let chunk_type = u32::from_le_bytes(buffer[offset + 4..offset + 8].try_into().unwrap()).into();
    let data = buffer[offset + 8..offset + 8 + length as usize].to_vec();

    Chunk {
        length,
        chunk_type,
        data,
    }
}
