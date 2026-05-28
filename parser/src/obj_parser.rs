use geometry::{HittableType, Mesh, Tri};
use material::{LambertianBase, Material, MaterialType};
use util::Vec3;

use crate::mtl_parser::parse_mtl;

#[allow(dead_code, clippy::too_many_lines)]
pub fn parse_obj(path: &str) -> (Vec<HittableType>, Vec<MaterialType>) {
    let file = std::fs::read_to_string(path).expect("Failed to read .obj file");
    let mut vertices: Vec<Vec3> = vec![];
    let mut v_normals: Vec<Vec3> = vec![];
    let mut v_textures: Vec<Vec3> = vec![];
    let mut tris: Vec<Tri> = vec![];
    let mut materials = vec![MaterialType::Lambertian(LambertianBase {
        name: "default".to_string(),
        albedo: Vec3::new(1.0, 0.0, 1.0),
        normal_texture: None,
        orm: Vec3::new(1.0, 0.8, 0.0),
        alpha: 1.0,
    })];

    let mut current_material_index = 0;

    for line in file.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "mtllib" => {
                let mtl_file = parts.get(1).unwrap_or(&"");
                let mtl_path = std::path::Path::new(path)
                    .parent()
                    .unwrap_or(std::path::Path::new(""))
                    .join(mtl_file);
                materials = parse_mtl(mtl_path.to_str().unwrap_or(""));
            }
            "usemtl" => {
                let mtl_name = parts.get(1).unwrap_or(&"default");
                if let Some((idx, _)) = materials
                    .iter()
                    .enumerate()
                    .find(|(_, m)| m.get_name() == *mtl_name)
                {
                    current_material_index = idx;
                } else {
                    current_material_index = 0; // Default material
                }
            }
            "v" => {
                if parts.len() < 4 {
                    continue;
                }
                let x: f32 = parts[1].parse().unwrap_or(0.0);
                let y: f32 = parts[2].parse().unwrap_or(0.0);
                let z: f32 = parts[3].parse().unwrap_or(0.0);
                vertices.push(Vec3 { x, y, z });
            }
            "vn" => {
                if parts.len() < 4 {
                    continue;
                }
                let x: f32 = parts[1].parse().unwrap_or(0.0);
                let y: f32 = parts[2].parse().unwrap_or(0.0);
                let z: f32 = parts[3].parse().unwrap_or(0.0);
                v_normals.push(Vec3 { x, y, z }.normalize());
            }
            "vt" => {
                // Texture coordinate
                if parts.len() < 3 {
                    continue;
                }
                let u: f32 = parts[1].parse().unwrap_or(0.0);
                let v: f32 = parts[2].parse().unwrap_or(0.0);
                v_textures.push(Vec3 { x: u, y: v, z: 0.0 });
            }
            "f" => {
                if parts.len() != 4 {
                    dbg!("Skipping non-triangular face: {}", line);
                    continue;
                }

                let mut v = [Vec3::zero(); 3];
                let mut n = [None, None, None];
                let mut vt = [None, None, None];

                for i in 0..3 {
                    let indices: Vec<&str> = parts[i + 1].split('/').collect();

                    // Vertex position
                    if let Some(v_str) = indices.first()
                        && let Ok(v_idx) = v_str.parse::<usize>()
                    {
                        v[i] = vertices[v_idx - 1]; // OBJ is 1-indexed
                    }

                    // Vertex normal
                    if let Some(vn_str) = indices.get(2)
                        && let Ok(vn_idx) = vn_str.parse::<usize>()
                    {
                        n[i] = Some(v_normals[vn_idx - 1]);
                    }

                    // Vertex texture coordinate
                    if let Some(vt_str) = indices.get(1)
                        && let Ok(vt_idx) = vt_str.parse::<usize>()
                    {
                        vt[i] = Some(v_textures[vt_idx - 1]);
                    }
                }

                let normals = if n.iter().all(|&norm| norm.is_some()) {
                    Some((n[0].unwrap(), n[1].unwrap(), n[2].unwrap()))
                } else {
                    None
                };

                let uvs = if vt.iter().all(|&tex| tex.is_some()) {
                    Some((vt[0].unwrap(), vt[1].unwrap(), vt[2].unwrap()))
                } else {
                    None
                };

                tris.push(Tri::new(
                    v[0],
                    v[1],
                    v[2],
                    normals,
                    uvs,
                    None,
                    Some(current_material_index),
                ));
            }

            _ => {}
        }
    }

    let hittables = tris.into_iter().map(HittableType::Tri).collect();
    let mesh = Mesh::new(hittables);
    let objects = vec![HittableType::Mesh(mesh)];

    (objects, materials)
}
