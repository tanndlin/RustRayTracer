use std::sync::Arc;

use crate::{
    geometry::{
        bounds::Bounds,
        hittable::{Hittable, HittableType},
    },
    util::{
        hit_result::HitResult,
        interval::Interval,
        parser::glb::gltf::Node,
        quat::{quat_inverse, quat_rotate},
        ray::Ray,
        vec3::{Vec3, max, min},
    },
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Instance {
    pub name: String,
    pub translation: Option<Vec3>,
    pub rotation: Option<[f32; 4]>,
    pub scale: Option<Vec3>,
    bounds: Bounds, // World Space
    pub base: Arc<HittableType>,
}

impl Instance {
    pub fn new(
        name: String,
        translation: Option<Vec3>,
        rotation: Option<[f32; 4]>,
        scale: Option<Vec3>,
        base: Arc<HittableType>,
    ) -> Self {
        Self {
            name,
            translation,
            rotation,
            scale,
            bounds: Self::calc_bounds(&base, translation, rotation, scale),
            base,
        }
    }
}

impl Hittable for Instance {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        self.get_bounds().hit(ray, interval)?;

        let mut origin = ray.origin;
        let mut dir = ray.dir;

        // Transform ray to object space (inverse transforms)
        if let Some(t) = self.translation {
            origin = origin - t;
        }
        if let Some(q) = self.rotation {
            let inv_q = quat_inverse(q);
            origin = quat_rotate(inv_q, origin);
            dir = quat_rotate(inv_q, dir);
        }
        if let Some(s) = self.scale {
            origin = origin / s;
            dir = dir / s;
        }

        let transformed_ray = Ray::new(origin, dir);
        let mut hit = self.base.hit(&transformed_ray, interval)?;

        if let Some(s) = self.scale {
            hit.point = hit.point * s;
            hit.normal = (hit.normal / s).normalize();
        }
        if let Some(q) = self.rotation {
            hit.point = quat_rotate(q, hit.point);
            hit.normal = quat_rotate(q, hit.normal);
        }
        if let Some(t) = self.translation {
            hit.point = hit.point + t;
        }
        Some(hit)
    }

    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn translate(&mut self, vec: &Vec3) {
        match self.translation {
            Some(t) => self.translation = Some(t + vec),
            None => self.translation = Some(*vec),
        };

        self.recompute_bounds();
    }
}

impl From<(&[Arc<HittableType>], Node)> for Instance {
    fn from(value: (&[Arc<HittableType>], Node)) -> Self {
        let (meshes, node) = value;
        let mesh_index = node
            .mesh
            .expect("GLTF node must have a mesh to be instanced");

        let translation = node.translation.map(Vec3::from);
        let rotation = node.rotation.map(|r| {
            let arr: [f64; 4] = r.try_into().unwrap();
            arr.map(|f| f as f32)
        });
        let scale = node.scale.map(Vec3::from);

        let base = meshes
            .get(mesh_index)
            .expect("Mesh index out of bounds for GLTF node")
            .clone();

        Self::new(node.name, translation, rotation, scale, base)
    }
}

impl Instance {
    #[allow(dead_code)]
    fn recompute_bounds(&mut self) {
        self.bounds = Self::calc_bounds(&self.base, self.translation, self.rotation, self.scale);
    }

    fn calc_bounds(
        base: &Arc<HittableType>,
        translation: Option<Vec3>,
        rotation: Option<[f32; 4]>,
        scale: Option<Vec3>,
    ) -> Bounds {
        let bounds = base.get_bounds();

        let corners = [
            Vec3::new(bounds.min.x, bounds.min.y, bounds.min.z),
            Vec3::new(bounds.max.x, bounds.min.y, bounds.min.z),
            Vec3::new(bounds.min.x, bounds.max.y, bounds.min.z),
            Vec3::new(bounds.min.x, bounds.min.y, bounds.max.z),
            Vec3::new(bounds.max.x, bounds.max.y, bounds.min.z),
            Vec3::new(bounds.max.x, bounds.min.y, bounds.max.z),
            Vec3::new(bounds.min.x, bounds.max.y, bounds.max.z),
            Vec3::new(bounds.max.x, bounds.max.y, bounds.max.z),
        ];

        let transformed: Vec<Vec3> = corners
            .iter()
            .map(|&c| {
                let mut p = c;
                if let Some(s) = scale {
                    p = p * s;
                }
                if let Some(q) = rotation {
                    p = quat_rotate(q, p);
                }
                if let Some(t) = translation {
                    p = p + t;
                }
                p
            })
            .collect();

        let min = transformed.iter().copied().reduce(min).unwrap();
        let max = transformed.iter().copied().reduce(max).unwrap();
        Bounds { min, max }
    }
}
