use crate::{
    geometry::{bounds::Bounds, hittable::Hittable},
    util::ray::Ray,
    util::vec3::{Vec3, cross, dot, max, min},
};

#[derive(Clone, Copy, Debug)]
pub struct Tri {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    normal: Vec3,
    edge_ab: Vec3,
    edge_ac: Vec3,
}

impl Tri {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        let edge_ab = v1.sub(v0);
        let edge_ac = v2.sub(v0);
        let normal = cross(edge_ab, edge_ac);

        Tri {
            v0,
            v1,
            v2,
            normal,
            edge_ab,
            edge_ac,
        }
    }
}

impl Hittable for Tri {
    fn hit(&self, r: &Ray) -> bool {
        let ao = r.origin.sub(self.v0);
        let dao = cross(ao, r.dir);

        // Backface culling
        let determinant = -dot(r.dir, self.normal);
        if determinant < 1e-6 {
            return false;
        }

        let inv_det = 1.0 / determinant;

        // Calculate dst to triangle
        let dst = dot(ao, self.normal) * inv_det;
        if dst < 0.0 {
            return false;
        }

        let u = dot(self.edge_ac, dao) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return false;
        }

        let v = -dot(self.edge_ab, dao) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        // if !ray_t.contains(dst){
        //     return false;
        // }

        true
    }

    fn get_bounds(&self) -> Bounds {
        let min = min(self.v0, min(self.v1, self.v2));
        let max = max(self.v0, max(self.v1, self.v2));

        Bounds { min, max }
    }
}
