use rayon::prelude::*;
use std::f64::consts::PI;

const MAX_BOUNCES: u32 = 10;
const USE_MULTITHREADING: bool = true;

use crate::{
    geometry::hittable::Hittable,
    material::{lambertian::Lambertian, material_trait::Material},
    util::{
        hit_result::HitResult,
        interval::Interval,
        ray::Ray,
        vec3::{Color, Vec3, cross},
    },
};

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    fov: u8,
    look_from: Vec3,
    look_at: Vec3,
    up: Vec3,
    materials: Vec<Lambertian>,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        let materials = vec![
            Lambertian {
                albedo: Color::new(0.8, 0.3, 0.3),
            },
            Lambertian {
                albedo: Color::new(0.8, 0.8, 0.0),
            },
            Lambertian {
                albedo: Color::new(0.8, 0.6, 0.2),
            },
            Lambertian {
                albedo: Color::new(0.1, 0.2, 0.5),
            },
        ];

        Camera {
            image_width,
            image_height,
            fov: 65,
            look_from: Vec3::new(-3.0, 0.5, -2.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            materials,
        }
    }

    pub fn render(&self, objects: &Vec<Box<dyn Hittable + Sync>>) -> Vec<Color> {
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

        match USE_MULTITHREADING {
            true => (0..self.image_height)
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
                .collect(),
            false => (0..self.image_height)
                .flat_map(|y| {
                    (0..self.image_width).map(move |x| {
                        let pixel_loc = pixel00_loc
                            .add(pixel_delta_u.scale(x as f64))
                            .add(pixel_delta_v.scale(y as f64));
                        let dir = pixel_loc.sub(self.look_from).normalize();
                        let ray = Ray::new(self.look_from, dir);

                        self.ray_color(&ray, objects, 0)
                    })
                })
                .collect(),
        }
    }

    fn ray_color(&self, ray: &Ray, objects: &Vec<Box<dyn Hittable + Sync>>, depth: u8) -> Color {
        if depth >= MAX_BOUNCES as u8 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut hit_result: Option<HitResult> = None;
        let mut interval = Interval {
            min: 0.001,
            max: f64::INFINITY,
        };

        for object in objects {
            if let Some(hit) = object.hit(ray, &interval)
                && (hit_result.is_none() || hit.t < hit_result.as_ref().unwrap().t)
            {
                interval.max = hit.t;
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
        Color::new(1.0 - t, 1.0 - t, 1.0 - t).add(Color::new(0.5, 0.7, 1.0).scale(t))
    }
}

fn degrees_to_radians(fov: u8) -> f64 {
    fov as f64 / 180.0 * PI
}
