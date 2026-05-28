use std::{io::Read, sync::Arc};

use crate::{
    geometry::hittable::HittableType,
    material::{
        dielectric::Dielectric, lambertian::LambertianBase, material_trait::MaterialType,
        texture::Texture,
    },
    util::{
        parser::glb::{
            gltf::{GltfData, Material, MimeType},
            types::{Chunk, ChunkType, GlbHeader},
        },
        vec3::{Color, Vec3},
    },
};

pub fn parse_glb(path: &str, mat_offset: usize) -> (Vec<HittableType>, Vec<MaterialType>) {
    let mut buffer = vec![];
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

    assemble_scene(gltf_data, binary_chunk, mat_offset)
}

fn assemble_scene(
    mut gltf_data: GltfData,
    binary_chunk: &Chunk,
    mat_offset: usize,
) -> (Vec<HittableType>, Vec<MaterialType>) {
    let scene = gltf_data
        .scenes
        .get(gltf_data.scene)
        .expect("Scene index out of bounds");

    let instance_bases = gltf_data
        .meshes
        .iter()
        .map(|mesh| {
            Arc::new(HittableType::from_gltf_mesh(
                mesh,
                &gltf_data,
                &binary_chunk.data,
                mat_offset,
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
                .get(node_index)
                .expect("Node index out of bounds")
        })
        .collect::<Vec<_>>();

    let instances: Vec<HittableType> = nodes
        .iter()
        .map(|node| {
            HittableType::Instance(Box::new(
                (instance_bases.as_slice(), (*node).clone()).into(),
            ))
        })
        .collect();

    let mut materials = vec![];
    let materials_data = std::mem::take(&mut gltf_data.materials);
    for mat in materials_data {
        let Material {
            name,
            normal_texture,
            pbr_metallic_roughness: pbr,
            ..
        } = mat;

        let pbr = pbr.unwrap();

        let normal_texture =
            normal_texture.map(|tex| load_texture(&binary_chunk.data, &gltf_data, tex.index));

        let roughness_texture = pbr
            .metallic_roughness_texture
            .map(|tex| load_texture(&binary_chunk.data, &gltf_data, tex.index));

        let material = {
            // Is glass
            if let Some(transmission_factor) =
                mat.extensions.transmission.map(|t| t.transmission_factor)
            {
                let ior = mat.extensions.ior.map_or(1.5, |i| i.ior);
                let albedo = pbr.base_color_factor.unwrap_or(vec![1.0, 1.0, 1.0, 1.0]);
                MaterialType::Dielectric(Dielectric::new(
                    name,
                    Some(albedo[..3].into()),
                    ior as f32,
                    transmission_factor as f32,
                ))
            } else {
                // Is lambertian
                if let Some(tex) = pbr.base_color_texture {
                    let albedo = load_texture(&binary_chunk.data, &gltf_data, tex.index);
                    let roughness = roughness_texture.unwrap_or({
                        let pixels = albedo
                            .data
                            .iter()
                            .map(|_| pbr.roughness_factor.map_or(0.8, |r| r as f32))
                            .map(|r| Vec3::new(0.0, r, 0.0))
                            .collect();

                        Texture {
                            data: pixels,
                            width: albedo.width,
                            height: albedo.height,
                        }
                    });

                    MaterialType::TextureLambertian(LambertianBase {
                        name,
                        albedo,
                        normal_texture,
                        orm: roughness,
                        alpha: 1.0,
                    })
                } else {
                    let rgba = pbr.base_color_factor.unwrap();
                    MaterialType::Lambertian(LambertianBase {
                        name,
                        albedo: rgba[..3].into(),
                        normal_texture,
                        orm: Vec3::new(1.0, pbr.roughness_factor.unwrap() as f32, 0.0),
                        alpha: rgba[3] as f32,
                    })
                }
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
        r#type: chunk_type,
        data,
    }
}

fn load_texture(binary: &[u8], gltf_data: &GltfData, tex_index: usize) -> Texture {
    let texture = gltf_data.textures.get(tex_index).unwrap();
    let image = gltf_data.images.get(texture.source).unwrap();
    let buffer_view = gltf_data.buffer_views.get(image.buffer_view).unwrap();
    let data = &binary[buffer_view.byte_offset..buffer_view.byte_offset + buffer_view.byte_length];

    let image = match image.mime_type {
        MimeType::ImagePng => image::load_from_memory_with_format(data, image::ImageFormat::Png)
            .expect("Failed to load PNG texture")
            .to_rgba8(),
        MimeType::ImageJpeg => image::load_from_memory_with_format(data, image::ImageFormat::Jpeg)
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
