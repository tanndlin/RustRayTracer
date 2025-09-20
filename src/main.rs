use crate::camera::Camera;
use crate::geometry::hittable::HittableType;
use crate::geometry::{hittable::Hittable, mesh::Mesh, sphere::Sphere};
use crate::util::vec3::Vec3;

mod camera;
mod geometry;
mod material;
mod obj_parser;
mod util;

fn main() {
    let sphere = Sphere::new(Vec3::new(5.0, 0.0, 0.0), 1.0, 0);

    let (tris, materials) = obj_parser::parse_obj("src/objs/Chess/Chess.obj");
    let mut mesh = Mesh::new(tris);
    mesh.translate(&Vec3::new(0.0, -1.5, 0.0));

    let objects: Vec<HittableType> = vec![HittableType::Sphere(sphere), HittableType::Mesh(mesh)];
    let camera = Camera::new(16.0 / 9.0, 1000, materials);
    println!("Rendering...");

    let start = std::time::Instant::now();
    let framebuffer = camera.render(&objects);
    let duration = start.elapsed();
    println!("Render time: {:?}", duration);

    let file = "output.png";
    image::save_buffer(
        file,
        &framebuffer
            .iter()
            .flat_map(|c| {
                let ir = (255.999 * c.x.clamp(0.0, 0.999)) as u8;
                let ig = (255.999 * c.y.clamp(0.0, 0.999)) as u8;
                let ib = (255.999 * c.z.clamp(0.0, 0.999)) as u8;
                [ir, ig, ib]
            })
            .collect::<Vec<u8>>(),
        camera.image_width,
        camera.image_height,
        image::ColorType::Rgb8,
    )
    .unwrap();
    println!("Saved to {}", file);
}
