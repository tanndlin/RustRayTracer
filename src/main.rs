use crate::{aabb::AABB, camera::Camera, mesh::Mesh, sphere::Sphere};

mod aabb;
mod bounds;
mod camera;
mod hittable;
mod mesh;
mod obj_parser;
mod ray;
mod sphere;
mod tri;
mod vec3;

fn main() {
    let camera = Camera::new(16.0 / 9.0, 400);
    let sphere = Sphere {
        center: vec3::Vec3 {
            x: 5.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 1.0,
    };

    let tris = obj_parser::parse_obj("src/Chess.obj");
    let mesh = Mesh::new(tris);
    let aabb = AABB::new(mesh);

    let objects: Vec<Box<dyn hittable::Hittable>> = vec![Box::new(sphere), Box::new(aabb)];
    println!("Rendering...");
    let framebuffer = camera.render(&objects);

    let file = "output.ppm";
    println!("Writing to {}", file);

    let mut ppm = format!("P3\n{} {}\n255\n", camera.image_width, camera.image_height);
    for pixel in framebuffer {
        let ir = (255.999 * pixel.x) as i32;
        let ig = (255.999 * pixel.y) as i32;
        let ib = (255.999 * pixel.z) as i32;
        ppm.push_str(&format!("{} {} {}\n", ir, ig, ib));
    }
    std::fs::write(file, ppm).expect("Unable to write file");
}
