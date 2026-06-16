use std::{path::Path, sync::Arc};

use geometry::{HittableType, Instance, Parent};
use gltf::{GltfData, Material, Node};
use material::{Dielectric, LambertianBase, MaterialType, Texture};
use util::Vec3;

use crate::glb::glb_parser::load_texture;

pub fn parse_gltf(path: &str, mat_offset: usize) -> (Vec<HittableType>, Vec<MaterialType>) {
    let gltf_data = std::fs::read_to_string(path).expect("Failed to read .gltf file");
    let gltf_data: GltfData = serde_json::from_str(&gltf_data).expect("Failed to parse .gltf file");

    // prepend the directory of the gltf file to the buffer uris
    let base_path = std::path::Path::new(path)
        .parent()
        .expect("Failed to get parent directory of .gltf file");

    let buffers = gltf_data
        .buffers
        .iter()
        .map(|buffer| {
            let uri = buffer.uri.as_ref().expect("Buffer URI is missing");
            let buffer_path = base_path.join(uri);
            std::fs::read(buffer_path).expect("Failed to read buffer file")
        })
        .collect::<Vec<_>>();

    let binary = buffers.iter().map(Vec::as_slice).collect::<Vec<_>>();
    assemble_scene(gltf_data, &binary, mat_offset, base_path)
}

pub fn assemble_scene(
    gltf_data: GltfData,
    binary_chunk: &[&[u8]],
    mat_offset: usize,
    base_path: &Path,
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
                binary_chunk,
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
        .filter_map(|node| {
            parse_node(node, &gltf_data, &instance_bases)
                .unwrap()
                .into()
        })
        .collect();

    println!("Parsed {} instances", instances.len());

    let materials = parse_materials(gltf_data, binary_chunk, base_path);

    (instances, materials)
}

fn parse_materials(
    mut gltf_data: GltfData,
    binary_chunk: &[&[u8]],
    base_path: &Path,
) -> Vec<MaterialType> {
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
            normal_texture.map(|tex| load_texture(binary_chunk, &gltf_data, tex.index, base_path));

        let roughness_texture = pbr
            .metallic_roughness_texture
            .map(|tex| load_texture(binary_chunk, &gltf_data, tex.index, base_path));

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
                    let albedo = load_texture(binary_chunk, &gltf_data, tex.index, base_path);
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
                    let rgba = pbr.base_color_factor.unwrap_or(vec![1.0, 1.0, 1.0, 1.0]);
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
    materials
}

fn parse_parent(node: &Node, gltf_data: &GltfData, instance_bases: &[Arc<HittableType>]) -> Parent {
    let children = node
        .children
        .as_ref()
        .expect("Node has no children")
        .iter()
        .filter_map(|&child_index| {
            let child_node = gltf_data
                .nodes
                .get(child_index)
                .expect("Child node index out of bounds");
            parse_node(child_node, gltf_data, instance_bases).ok()
        })
        .collect();

    let rotation = node.rotation.as_ref().map(|r| {
        let arr: &[f64; 4] = r.as_slice().try_into().unwrap();
        arr.map(|v| v as f32)
    });

    let object_to_world = node.matrix.map(|matrix| {
        [
            [matrix[0], matrix[4], matrix[8], matrix[12]],
            [matrix[1], matrix[5], matrix[9], matrix[13]],
            [matrix[2], matrix[6], matrix[10], matrix[14]],
            [matrix[3], matrix[7], matrix[11], matrix[15]],
        ] as [[f64; 4]; 4]
    });

    Parent::new(
        node.name.clone(),
        node.translation.clone().map(Vec3::from),
        rotation,
        node.scale.clone().map(Vec3::from),
        object_to_world,
        children,
    )
}

fn parse_node(
    node: &Node,
    gltf_data: &GltfData,
    instance_bases: &[Arc<HittableType>],
) -> Result<HittableType, String> {
    if node.children.is_some() {
        Ok(HittableType::Parent(Box::new(parse_parent(
            node,
            gltf_data,
            instance_bases,
        ))))
    } else {
        Instance::try_from((instance_bases, node.clone()))
            .map(|instance| HittableType::Instance(Box::new(instance)))
            .map_err(|e| format!("Failed to parse node {}: {}", node.name, e))
    }
}
