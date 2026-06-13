#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]

use clap::Parser;
use geometry::Hittable;
use parser::parse_glb;
use util::Vec3;

use crate::camera::Camera;

mod camera;
mod progress;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "10")]
    pub samples: u32,
    #[arg(long, default_value = "false")]
    pub debug_aabb: bool,
}

fn main() {
    let args = Args::parse();

    let (mut objects, materials) = parse_glb("objs/Titanic/combined.glb", 0);

    objects[0].scale(&Vec3::new(0.25, 0.25, 0.25));
    objects[0].translate(&Vec3::new(-30.0, -5.0, 0.0));

    let camera = Camera::new(
        16.0 / 9.0,
        1000,
        args.samples,
        materials,
        true,
        args.debug_aabb,
    );
    println!("Rendering...");

    let start = std::time::Instant::now();
    let framebuffer = camera.render(objects);
    let duration = start.elapsed();
    println!("Render time: {duration:?}");

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
    println!("Saved to {file}");
}
