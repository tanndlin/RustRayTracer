use rayon::prelude::*;
use std::f32::consts::PI;

const MAX_BOUNCES: u32 = 10;
const TILE_SIZE: u32 = 16;
const SAMPLES_PER_PIXEL: u32 = 10;

use crate::{
    geometry::hittable::{Hittable, HittableType},
    material::material_trait::{Material, MaterialType},
    util::{
        hit_result::HitResult,
        interval::Interval,
        progress::progress_bar,
        ray::Ray,
        vec3::{Color, Vec3, cross},
    },
};

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    look_from: Vec3,
    materials: Vec<MaterialType>,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    total_pixels: u32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32, materials: Vec<MaterialType>) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u32;

        let look_from = Vec3::new(-3.0, 0.5, -2.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let fov = 55;

        let theta = degrees_to_radians(fov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * image_width as f32 / image_height as f32;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).normalize();
        let u = cross(up, w).normalize();
        let v = cross(w, u);

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

        Camera {
            image_width,
            image_height,
            look_from,
            materials,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            total_pixels,
        }
    }

    pub fn render(&self, objects: &Vec<HittableType>) -> Vec<Color> {
        // Determine viewport dimensions.
        let num_tiles =
            (self.total_pixels as f32 / TILE_SIZE as f32 / TILE_SIZE as f32).ceil() as u32;

        // Each tile is a square of TILE_SIZE x TILE_SIZE pixels
        let tiles = self.collect_tiles(num_tiles, objects);
        println!(); // New line after progress output
        tiles.into_iter().flatten().collect()
    }

    #[cfg(feature = "multithreading")]
    fn collect_tiles(&self, num_tiles: u32, objects: &Vec<HittableType>) -> Vec<Vec<Vec3>> {
        progress_bar(0..num_tiles)
            .into_par_iter()
            .map(|tile_index| self.render_tile(tile_index, objects))
            .collect()
    }

    #[cfg(not(feature = "multithreading"))]
    fn collect_tiles(&self, num_tiles: u32, objects: &Vec<HittableType>) -> Vec<Vec<Vec3>> {
        progress_bar(0..num_tiles)
            .map(|tile_index| self.render_tile(tile_index, objects))
            .collect()
    }

    fn render_tile(&self, tile_index: u32, objects: &Vec<HittableType>) -> Vec<Vec3> {
        let mut tile_buffer = vec![];
        let start_pixel = tile_index * TILE_SIZE * TILE_SIZE;
        let end_pixel = ((tile_index + 1) * TILE_SIZE * TILE_SIZE).min(self.total_pixels);
        for pixel_index in start_pixel..end_pixel {
            let i = pixel_index % self.image_width;
            let j = pixel_index / self.image_width;
            let pixel_center =
                self.pixel00_loc + self.pixel_delta_u * i as f32 + self.pixel_delta_v * j as f32;
            let ray = Ray::new(self.look_from, (pixel_center - self.look_from).normalize());

            let mut color = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                color = color + self.ray_color(&ray, objects);
            }
            tile_buffer.push(color / SAMPLES_PER_PIXEL as f32);
        }
        tile_buffer
    }

    fn ray_color(&self, ray: &Ray, objects: &Vec<HittableType>) -> Color {
        let mut depth = 0;
        let mut attenuation = Color::new(1.0, 1.0, 1.0);
        let mut ray = *ray;
        while depth < MAX_BOUNCES {
            let mut hit_result: Option<HitResult> = None;
            let mut interval = Interval {
                min: 0.001,
                max: f32::INFINITY,
            };

            for object in objects {
                if let Some(hit) = object.hit(&ray, &interval)
                    && (hit_result.is_none() || hit.t < hit_result.as_ref().unwrap().t)
                {
                    interval.max = hit.t;
                    hit_result = Some(hit);
                }
            }

            if let Some(hit) = hit_result {
                let material = &self.materials[hit.material_index];
                let (scattered_ray, new_color) = material.scatter(&ray, &hit);
                attenuation = attenuation * new_color;
                ray = scattered_ray;
                depth += 1;
            } else {
                // Background gradient
                let unit_direction = ray.dir.normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                return attenuation
                    * (Color::new(1.0 - t, 1.0 - t, 1.0 - t) + Color::new(0.5, 0.7, 1.0) * t);
            }
        }

        attenuation
    }
}

fn degrees_to_radians(fov: u8) -> f32 {
    fov as f32 / 180.0 * PI
}
