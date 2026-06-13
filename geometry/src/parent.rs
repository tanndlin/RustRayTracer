use util::{HitResult, Interval, Ray, Vec3, quat::from_axis_angle};

use crate::{
    Bounds, Hittable, HittableType,
    transpose::{
        mat3_inverse_transpose, mat4_inverse, mat4_multiply, mat4_transform_dir,
        mat4_transform_point, transform_bounds_with_matrix, trs_matrix,
    },
};

#[derive(Debug)]
pub struct Parent {
    pub name: String,
    pub children: Vec<HittableType>,
    pub world_to_object: [[f64; 4]; 4],
    pub object_to_world: [[f64; 4]; 4],
    normal_matrix: [[f64; 4]; 4],
    local_bounds: Bounds, // children bounds in parent-local space
    bounds: Bounds,       // world-space bounds
}

impl Parent {
    pub fn new(
        name: String,
        translation: Option<Vec3>,
        rotation: Option<[f32; 4]>,
        scale: Option<Vec3>,
        object_to_world: Option<[[f64; 4]; 4]>,
        children: Vec<HittableType>,
    ) -> Self {
        let scale = scale.unwrap_or(Vec3::from(1.0));

        let object_to_world =
            object_to_world.unwrap_or_else(|| trs_matrix(translation, rotation, scale));
        let world_to_object = mat4_inverse(object_to_world);
        let normal_matrix = mat3_inverse_transpose(object_to_world);

        let local_bounds = Bounds::from(&children);
        let bounds = transform_bounds_with_matrix(&local_bounds, object_to_world);

        Parent {
            name,
            children,
            world_to_object,
            object_to_world,
            normal_matrix,
            local_bounds,
            bounds,
        }
    }

    fn apply_transform(&mut self, inc: [[f64; 4]; 4]) {
        self.object_to_world = mat4_multiply(self.object_to_world, inc); // swap order
        self.world_to_object = mat4_inverse(self.object_to_world);
        self.normal_matrix = mat3_inverse_transpose(self.object_to_world);
        self.bounds = transform_bounds_with_matrix(&self.local_bounds, self.object_to_world);
    }
}

impl Hittable for Parent {
    fn hit(&self, ray: &util::Ray, interval: &util::Interval) -> Option<util::HitResult> {
        self.get_bounds().hit(ray, interval)?;

        let origin = mat4_transform_point(self.world_to_object, ray.origin);
        let dir_transformed = mat4_transform_dir(self.world_to_object, &ray.dir);
        let dir_length = dir_transformed.length();

        let transformed_ray = Ray::new(origin, dir_transformed.normalize());
        let transformed_interval = Interval {
            min: interval.min * dir_length,
            max: interval.max * dir_length,
        };

        let mut closest_hit: Option<HitResult> = None;
        for child in &self.children {
            if let Some(mut hit) = child.hit(&transformed_ray, &transformed_interval) {
                // t is in object space with normalized dir, scale back to world space
                hit.t /= dir_length;

                hit.point = mat4_transform_point(self.object_to_world, hit.point);
                hit.normal = mat4_transform_dir(self.normal_matrix, &hit.normal).normalize();

                if closest_hit.is_none() || hit.t < closest_hit.as_ref().unwrap().t {
                    closest_hit = Some(hit);
                }
            }
        }

        closest_hit
    }

    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn translate(&mut self, vec: &Vec3) {
        self.apply_transform([
            [1.0, 0.0, 0.0, f64::from(vec.x)],
            [0.0, 1.0, 0.0, f64::from(vec.y)],
            [0.0, 0.0, 1.0, f64::from(vec.z)],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    fn scale(&mut self, vec: &Vec3) {
        self.apply_transform([
            [f64::from(vec.x), 0.0, 0.0, 0.0],
            [0.0, f64::from(vec.y), 0.0, 0.0],
            [0.0, 0.0, f64::from(vec.z), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    fn rotate(&mut self, axis: &Vec3, angle_rad: f32) {
        let [qx, qy, qz, qw] = from_axis_angle(*axis, angle_rad).map(f64::from);
        self.apply_transform([
            [
                1.0 - 2.0 * (qy * qy + qz * qz),
                2.0 * (qx * qy - qz * qw),
                2.0 * (qx * qz + qy * qw),
                0.0,
            ],
            [
                2.0 * (qx * qy + qz * qw),
                1.0 - 2.0 * (qx * qx + qz * qz),
                2.0 * (qy * qz - qx * qw),
                0.0,
            ],
            [
                2.0 * (qx * qz - qy * qw),
                2.0 * (qy * qz + qx * qw),
                1.0 - 2.0 * (qx * qx + qy * qy),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    fn debug_hit_count(&self, ray: &util::Ray, interval: &util::Interval) -> u32 {
        self.children
            .iter()
            .map(|child| child.debug_hit_count(ray, interval))
            .sum()
    }
}
