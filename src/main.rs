use clap::Parser;

use crate::{camera::Camera, util::parser::glb::glb_parser::parse_glb};

mod camera;
mod geometry;
mod material;
mod util;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "10")]
    pub samples: u32,
}

fn main() {
    let args = Args::parse();

    let (objects, materials) = parse_glb("src/objs/Chess/Chess.glb");

    let camera = Camera::new(16.0 / 9.0, 1000, args.samples, materials, true);
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
