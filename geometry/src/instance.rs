use std::sync::Arc;

use gltf::Node;
use util::{
    HitResult, Interval, Ray, Vec3,
    quat::{from_axis_angle, quat_multiply},
};

use crate::{
    bounds::Bounds,
    hittable::{Hittable, HittableType},
    transpose::{
        mat3_inverse_transpose, mat4_inverse, mat4_transform_dir, mat4_transform_point,
        transform_bounds_with_matrix, trs_matrix,
    },
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Instance {
    pub name: String,
    pub translation: Option<Vec3>,
    pub rotation: Option<[f32; 4]>,
    pub scale: Vec3,
    pub world_to_object: [[f64; 4]; 4], // inverse TRS
    pub object_to_world: [[f64; 4]; 4], // TRS
    normal_matrix: [[f64; 4]; 4],
    bounds: Bounds, // World Space
    pub base: Arc<HittableType>,
}

impl Instance {
    pub fn new(
        name: String,
        base: Arc<HittableType>,
        translation: Option<Vec3>,
        rotation: Option<[f32; 4]>,
        scale: Option<Vec3>,
        object_to_world: Option<[[f64; 4]; 4]>,
    ) -> Self {
        let scale = scale.unwrap_or(Vec3::from(1.0));

        let object_to_world =
            object_to_world.unwrap_or_else(|| trs_matrix(translation, rotation, scale));
        let world_to_object = mat4_inverse(object_to_world);
        let normal_matrix = mat3_inverse_transpose(object_to_world);

        Self {
            name,
            translation,
            rotation,
            scale,
            object_to_world,
            world_to_object,
            normal_matrix,
            bounds: transform_bounds_with_matrix(base.get_bounds(), object_to_world),
            base,
        }
    }
}

impl Hittable for Instance {
    fn debug_hit_count(&self, ray: &Ray, interval: &Interval) -> u32 {
        if self.get_bounds().hit(ray, interval).is_none() {
            return 0;
        }
        let origin = mat4_transform_point(self.world_to_object, ray.origin);
        let dir_transformed = mat4_transform_dir(self.world_to_object, &ray.dir);
        let dir_length = dir_transformed.length();
        let transformed_ray = Ray::new(origin, dir_transformed.normalize());
        let transformed_interval = Interval {
            min: interval.min * dir_length,
            max: interval.max * dir_length,
        };
        self.base
            .debug_hit_count(&transformed_ray, &transformed_interval)
    }

    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        self.get_bounds().hit(ray, interval)?;

        let origin = mat4_transform_point(self.world_to_object, ray.origin);
        let dir_transformed = mat4_transform_dir(self.world_to_object, &ray.dir);
        let dir_length = dir_transformed.length();

        let transformed_ray = Ray::new(origin, dir_transformed.normalize());
        let transformed_interval = Interval {
            min: interval.min * dir_length,
            max: interval.max * dir_length,
        };

        let mut hit = self
            .base
            .hit(&transformed_ray, &transformed_interval)?;

        // t is in object space with normalized dir, scale back to world space
        hit.t /= dir_length;

        hit.point = mat4_transform_point(self.object_to_world, hit.point);
        hit.normal = mat4_transform_dir(self.normal_matrix, &hit.normal).normalize();
        Some(hit)
    }

    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn translate(&mut self, vec: &Vec3) {
        match self.translation {
            Some(t) => self.translation = Some(t + vec),
            None => self.translation = Some(*vec),
        }

        self.recompute_bounds();
    }

    fn scale(&mut self, vec: &Vec3) {
        self.scale = self.scale * *vec;
        self.recompute_bounds();
    }

    fn rotate(&mut self, axis: &Vec3, angle_rad: f32) {
        let rot = from_axis_angle(*axis, angle_rad);
        self.rotation = match self.rotation {
            Some(r) => Some(quat_multiply(rot, r)),
            None => Some(rot),
        };

        self.recompute_bounds();
    }
}

impl TryFrom<(&[Arc<HittableType>], Node)> for Instance {
    type Error = String;

    fn try_from(base_meshes: (&[Arc<HittableType>], Node)) -> Result<Self, Self::Error> {
        let (meshes, node) = base_meshes;
        let mesh_index = node
            .mesh
            .ok_or_else(|| "GLTF node does not have a mesh".to_string())?;

        let translation = node.translation.map(Vec3::from);
        let rotation = node.rotation.map(|r| {
            let arr: [f64; 4] = r.try_into().unwrap();
            arr.map(|f| f as f32)
        });
        let scale = node.scale.map(Vec3::from);

        let object_to_world = node.matrix.map(|matrix| {
            [
                [matrix[0], matrix[4], matrix[8], matrix[12]],
                [matrix[1], matrix[5], matrix[9], matrix[13]],
                [matrix[2], matrix[6], matrix[10], matrix[14]],
                [matrix[3], matrix[7], matrix[11], matrix[15]],
            ] as [[f64; 4]; 4]
        });

        let base = meshes
            .get(mesh_index)
            .expect("Mesh index out of bounds for GLTF node")
            .clone();

        Ok(Self::new(
            node.name,
            base,
            translation,
            rotation,
            scale,
            object_to_world,
        ))
    }
}

impl Instance {
    #[allow(dead_code)]
    fn recompute_bounds(&mut self) {
        self.object_to_world = trs_matrix(self.translation, self.rotation, self.scale);
        self.world_to_object = mat4_inverse(self.object_to_world);
        self.normal_matrix = mat3_inverse_transpose(self.object_to_world);
        self.bounds = transform_bounds_with_matrix(self.base.get_bounds(), self.object_to_world);
    }
}
