use std::f32::consts::PI;

const MAX_BOUNCES: u32 = 100;
const TILE_SIZE: u32 = 16;

use geometry::{AABB, Hittable, HittableType};
use material::{LambertianBase, Material, MaterialType};
use util::{Color, Interval, Point, Ray, Unnormalized, Vec3};

use crate::progress::make_progress_bar;

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    samples_per_pixel: u32,
    look_from: Point,
    materials: Vec<MaterialType>,
    default_material: MaterialType,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    total_pixels: u32,
    use_background_gradient: bool,
    pub debug_aabb: bool,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
        materials: Vec<MaterialType>,
        use_background_gradient: bool,
        debug_aabb: bool,
    ) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u32;

        let look_from = Point::new(25.0, 20.0, -50.0);
        let look_at = Point::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0).normalize();
        let fov = 35;

        let theta = degrees_to_radians(fov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * image_width as f32 / image_height as f32;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).normalize();
        let u = Vec3::cross(&up, &w).normalize();
        let v = Vec3::cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width; // Vector across viewport horizontal edge
        let viewport_v = -v * viewport_height; // Vector down viewport vertical edge

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = look_from - w - viewport_u * 0.5 - viewport_v * 0.5;

        // Calculate the horizontal and vertical delta vectors to the next pixel.
        let pixel_delta_u = viewport_u * 1.0 / image_width as f32;
        let pixel_delta_v = viewport_v * 1.0 / image_height as f32;
        let pixel00_loc = viewport_upper_left + pixel_delta_u * 0.5 + pixel_delta_v * 0.5;

        let total_pixels = image_width * image_height;

        let default_material = MaterialType::Lambertian(LambertianBase {
            name: "Default".to_owned(),
            albedo: Color::new(1.0, 0.0, 1.0),
            normal_texture: None,
            orm: Vec3::new(1.0, 1.0, 0.0),
            alpha: 1.0,
        });

        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            look_from,
            materials,
            default_material,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            total_pixels,
            use_background_gradient,
            debug_aabb,
        }
    }

    pub fn render(&self, objects: Vec<HittableType>) -> Vec<Color> {
        // Create top-level node with BVH
        let aabb = AABB::new(objects);

        let num_tiles =
            (self.total_pixels as f32 / TILE_SIZE as f32 / TILE_SIZE as f32).ceil() as u32;
        self.collect_tiles(num_tiles, &aabb)
            .into_iter()
            .flatten()
            .collect()
    }

    #[cfg(feature = "multithreading")]
    fn collect_tiles(&self, num_tiles: u32, objects: &AABB) -> Vec<Vec<Vec3>> {
        use indicatif::ParallelProgressIterator;
        use rayon::prelude::*;

        (0..num_tiles)
            .into_par_iter()
            .progress_with(make_progress_bar(u64::from(num_tiles)))
            .map(|tile_index| self.render_tile(tile_index, objects))
            .collect()
    }

    #[cfg(not(feature = "multithreading"))]
    fn collect_tiles(&self, num_tiles: u32, objects: &AABB) -> Vec<Vec<Vec3>> {
        use indicatif::ProgressIterator;
        (0..num_tiles)
            .progress_with(make_progress_bar(num_tiles as u64))
            .map(|tile_index| self.render_tile(tile_index, objects))
            .collect()
    }

    fn render_tile(&self, tile_index: u32, objects: &AABB) -> Vec<Vec3> {
        let mut tile_buffer = vec![];
        let start_pixel = tile_index * TILE_SIZE * TILE_SIZE;
        let end_pixel = ((tile_index + 1) * TILE_SIZE * TILE_SIZE).min(self.total_pixels);
        for pixel_index in start_pixel..end_pixel {
            let i = pixel_index % self.image_width;
            let j = pixel_index / self.image_width;

            let pixel_center =
                self.pixel00_loc + self.pixel_delta_u * i as f32 + self.pixel_delta_v * j as f32;

            let ray_dir = (pixel_center - self.look_from).normalize();
            let mut color = Color::zero();
            for _ in 0..self.samples_per_pixel {
                let ray = Ray::new(self.look_from, ray_dir);
                color = color + self.ray_color(ray, objects);
            }
            let mut color = color / self.samples_per_pixel as f32;

            if self.debug_aabb {
                const AABB_ALPHA: f32 = 0.50;
                const MAX_COUNT: f32 = 1000.0;
                let interval = Interval {
                    min: 0.00001,
                    max: f32::INFINITY,
                };

                let debug_ray = Ray::new(self.look_from, ray_dir);
                let count = objects.debug_hit_count(&debug_ray, &interval);
                if count > 0 {
                    let t = (count as f32 / MAX_COUNT).min(1.0);
                    // cyan (few nodes) → red (many nodes)
                    let aabb_color = Color::new(t, 1.0 - t, 1.0 - t);
                    color = color * (1.0 - AABB_ALPHA) + aabb_color * AABB_ALPHA;
                }
            }

            tile_buffer.push(color);
        }
        tile_buffer
    }

    fn ray_color(&self, mut ray: Ray, objects: &AABB) -> Vec3<Unnormalized> {
        let mut depth = 0;
        let mut attenuation = Color::new(1.0, 1.0, 1.0);
        while depth < MAX_BOUNCES {
            let interval = Interval {
                min: 0.00001,
                max: f32::INFINITY,
            };

            if let Some(hit) = objects.hit(&ray, &interval) {
                let material = match hit.material_index {
                    Some(mat_index) => &self.materials[mat_index],
                    None => &self.default_material,
                };
                if matches!(material, MaterialType::Emissive(_)) {
                    return attenuation * material.scatter(&ray, &hit).1;
                }

                let (scattered_ray, new_color) = material.scatter(&ray, &hit);
                attenuation = attenuation * new_color;
                ray = scattered_ray;
                depth += 1;
            } else {
                // Background gradient
                if self.use_background_gradient {
                    let t = 0.5 * (ray.dir.y + 1.0);
                    return attenuation
                        * (Color::new(1.0 - t, 1.0 - t, 1.0 - t) + Color::new(0.5, 0.7, 1.0) * t);
                }

                return Color::zero();
            }
        }

        attenuation
    }
}

fn degrees_to_radians(fov: u8) -> f32 {
    f32::from(fov) / 180.0 * PI
}
