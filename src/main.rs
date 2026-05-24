use crate::{camera::Camera, util::parser::glb::glb_parser::parse_glb};

mod camera;
mod geometry;
mod material;
mod util;

fn main() {
    let (objects, materials) = parse_glb("src/objs/Chess/Chess.glb");

    let camera = Camera::new(16.0 / 9.0, 1000, materials, true);
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
