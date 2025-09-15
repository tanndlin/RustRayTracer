use crate::{geometry::tri::Tri, util::vec3::Vec3};

pub fn parse_obj(_path: &str) -> Vec<Tri> {
    let file = std::fs::read_to_string(_path).expect("Failed to read .obj file");
    let mut vertices: Vec<Vec3> = Vec::new();
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
            "f" => {
                if parts.len() < 4 {
                    continue;
                }
                let v_indices: Vec<usize> = parts[1..4]
                    .iter()
                    .filter_map(|p| p.split('/').next())
                    .filter_map(|idx| idx.parse::<usize>().ok())
                    .map(|idx| idx - 1) // OBJ indices are 1-based
                    .collect();

                if v_indices.len() == 3
                    && v_indices[0] < vertices.len()
                    && v_indices[1] < vertices.len()
                    && v_indices[2] < vertices.len()
                {
                    let tri = Tri::new(
                        vertices[v_indices[0]],
                        vertices[v_indices[1]],
                        vertices[v_indices[2]],
                    );
                    tris.push(tri);
                }
            }
            _ => {}
        }
    }

    tris
}
