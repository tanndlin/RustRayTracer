use crate::{
    geometry::tri::Tri,
    material::{
        lambertian::LambertianBase,
        material_trait::{Material, MaterialType},
    },
    util::vec3::{Color, Vec3},
};

pub fn parse_mtl(path: &str) -> Vec<MaterialType> {
    let file = std::fs::read_to_string(path).expect("Failed to read .mtl file");

    let mut materials = vec![];

    let mut cur_material_name = None;
    for line in file.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "newmtl" => {
                let name = parts.get(1).unwrap_or(&"default").to_string();
                cur_material_name = Some(name.clone());
                // Push a default material for now; we'll update it when we get more info
                materials.push(MaterialType::Lambertian(LambertianBase {
                    name,
                    albedo: Vec3::new(0.8, 0.8, 0.8),
                    roughness: 1.0,
                    alpha: 1.0,
                }));
            }
            "Kd" => {
                // Diffuse color
                if parts.len() < 4 {
                    continue;
                }
                let r: f32 = parts[1].parse().unwrap_or(0.8);
                let g: f32 = parts[2].parse().unwrap_or(0.8);
                let b: f32 = parts[3].parse().unwrap_or(0.8);
                if let Some(name) = cur_material_name.take()
                    && let Some(MaterialType::Lambertian(mat)) = materials.last_mut()
                {
                    mat.name = name;
                    mat.albedo = Vec3::new(r, g, b);
                }
            }
            "map_Kd" => {
                let file_name = parts.get(1).expect("Material missing file name");
                let path = std::path::Path::new(path);
                let file_name = path
                    .parent()
                    .unwrap_or(std::path::Path::new(""))
                    .join(file_name);
                println!("Loading texture: {:?}", file_name);
                let pixels: Vec<Color> = image::open(file_name)
                    .expect("Failed to open texture file")
                    .to_rgb8()
                    .pixels()
                    .map(|p| {
                        Color::new(
                            p[0] as f32 / 255.0,
                            p[1] as f32 / 255.0,
                            p[2] as f32 / 255.0,
                        )
                    })
                    .collect();

                if let Some(name) = cur_material_name.take()
                    && let Some(last) = materials.pop()
                {
                    match last {
                        MaterialType::Lambertian(mat) => {
                            let mut new_mat = LambertianBase::<Vec<Color>>::from(mat);
                            new_mat.name = name;
                            new_mat.albedo = pixels;
                            materials.push(MaterialType::TextureLambertian(new_mat));
                        }
                        other => {
                            // If it wasn't a Lambertian, just put it back unchanged
                            eprint!(
                                "Warning: Material type {:?} does not support textures. Skipping texture.",
                                other.get_name()
                            );
                            materials.push(other);
                        }
                    }
                }
            }
            "d" => {
                if parts.len() < 2 {
                    continue;
                }
                let alpha: f32 = parts[1].parse().unwrap_or(1.0);
                if let Some(MaterialType::Lambertian(mat)) = materials.last_mut() {
                    mat.alpha = alpha;
                }
            }
            _ => {}
        }
    }

    materials
}
