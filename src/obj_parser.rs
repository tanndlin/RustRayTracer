use crate::{geometry::tri::Tri, util::vec3::Vec3};

pub fn parse_obj(_path: &str) -> Vec<Tri> {
    let file = std::fs::read_to_string(_path).expect("Failed to read .obj file");
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut v_normals: Vec<Vec3> = Vec::new();
    let mut tris: Vec<Tri> = Vec::new();

    for line in file.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                if parts.len() < 4 {
                    continue;
                }
                let x: f64 = parts[1].parse().unwrap_or(0.0);
                let y: f64 = parts[2].parse().unwrap_or(0.0);
                let z: f64 = parts[3].parse().unwrap_or(0.0);
                vertices.push(Vec3 { x, y, z });
            }
            "vn" => {
                if parts.len() < 4 {
                    continue;
                }
                let x: f64 = parts[1].parse().unwrap_or(0.0);
                let y: f64 = parts[2].parse().unwrap_or(0.0);
                let z: f64 = parts[3].parse().unwrap_or(0.0);
                v_normals.push(Vec3 { x, y, z }.normalize());
            }
            "f" => {
                if parts.len() != 4 {
                    dbg!("Skipping non-triangular face: {}", line);
                    continue;
                }

                let mut v = [Vec3::new(); 3];
                let mut n = [None, None, None];

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
                }

                tris.push(Tri::new(v[0], v[1], v[2], n[0], n[1], n[2]));
            }

            _ => {}
        }
    }

    tris
}
