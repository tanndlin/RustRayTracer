use rayon::prelude::*;
use std::f64::consts::PI;

const MAX_BOUNCES: u32 = 10;

use crate::{
    geometry::hittable::Hittable,
    material::{lambertian::Lambertian, material_trait::Material},
    util::{
        hit_result::HitResult,
        ray::Ray,
        vec3::{Vec3, cross},
    },
};

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    aspect_ratio: f32,
    fov: u8,
    look_from: Vec3,
    look_at: Vec3,
    up: Vec3,
    pub materials: Vec<Lambertian>,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        let materials = vec![
            Lambertian {
                albedo: Vec3 {
                    x: 0.8,
                    y: 0.3,
                    z: 0.3,
                },
            },
            Lambertian {
                albedo: Vec3 {
                    x: 0.8,
                    y: 0.8,
                    z: 0.0,
                },
            },
            Lambertian {
                albedo: Vec3 {
                    x: 0.8,
                    y: 0.6,
                    z: 0.2,
                },
            },
            Lambertian {
                albedo: Vec3 {
                    x: 0.1,
                    y: 0.2,
                    z: 0.5,
                },
            },
        ];

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            fov: 90,
            look_from: Vec3::new(),
            look_at: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            up: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            materials,
        }
    }

    pub fn render(&self, objects: &Vec<Box<dyn Hittable + Sync>>) -> Vec<Vec3> {
        // Determine viewport dimensions.
        let theta = degrees_to_radians(self.fov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = self.look_from.sub(self.look_at).normalize();
        let u = cross(self.up, w).normalize();
        let v = cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u.scale(viewport_width); // Vector across viewport horizontal edge
        let viewport_v = v.negate().scale(viewport_height); // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors to the next pixel.
        let pixel_delta_u = viewport_u.scale(1.0 / self.image_width as f64);
        let pixel_delta_v = viewport_v.scale(1.0 / self.image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self
            .look_from
            .sub(w)
            .sub(viewport_u.scale(0.5))
            .sub(viewport_v.scale(0.5));

        let pixel00_loc = viewport_upper_left
            .add(pixel_delta_u.scale(0.5))
            .add(pixel_delta_v.scale(0.5));

        let frame_buffer: Vec<Vec3> = (0..self.image_height)
            .into_par_iter()
            .flat_map_iter(|y| {
                (0..self.image_width).map(move |x| {
                    let pixel_loc = pixel00_loc
                        .add(pixel_delta_u.scale(x as f64))
                        .add(pixel_delta_v.scale(y as f64));
                    let dir = pixel_loc.sub(self.look_from).normalize();
                    let ray = Ray::new(self.look_from, dir);

                    self.ray_color(&ray, objects, 0)
                })
            })
            .collect();

        frame_buffer
    }

    fn ray_color(&self, ray: &Ray, objects: &Vec<Box<dyn Hittable + Sync>>, depth: u8) -> Vec3 {
        if depth >= MAX_BOUNCES as u8 {
            return Vec3::new();
        }

        let mut hit_result: Option<HitResult> = None;

        for object in objects {
            if let Some(hit) = object.hit(ray)
                && (hit_result.is_none() || hit.t < hit_result.as_ref().unwrap().t)
            {
                hit_result = Some(hit);
            }
        }

        if let Some(hit) = hit_result {
            let material = &self.materials[hit.material_index];
            let (scattered_ray, attenuation) = material.scatter(ray, &hit);

            return attenuation.mul(self.ray_color(&scattered_ray, objects, depth + 1));
        }

        // Background gradient
        let unit_direction = ray.dir.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3 {
            x: (1.0 - t) + t * 0.5,
            y: (1.0 - t) + t * 0.7,
            z: (1.0 - t) + t * 1.0,
        }
    }
}

fn degrees_to_radians(fov: u8) -> f64 {
    fov as f64 / 180.0 * PI
}
