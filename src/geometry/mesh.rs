use crate::{
    geometry::{aabb::AABB, bounds::Bounds, hittable::Hittable, tri::Tri},
    util::{
        hit_result::HitResult,
        interval::Interval,
        parser::glb::{
            accessor::{self, AccessorData},
            gltf::{self, GltfData},
        },
        ray::Ray,
        vec3::Vec3,
    },
};

#[derive(Debug)]
pub struct Mesh {
    pub aabb: AABB<Tri>,
}

impl Mesh {
    pub fn new(children: Vec<Tri>) -> Self {
        let aabb = AABB::new(children);
        Mesh { aabb }
    }

    pub fn from_gltf_mesh(gltf_mesh: &gltf::Mesh, gltf_data: &GltfData, binary: &[u8]) -> Self {
        let mut children = Vec::new();

        for primitive in &gltf_mesh.primitives {
            let pos_accessor = gltf_data
                .accessors
                .get(primitive.attributes.position)
                .unwrap();
            let positions = match pos_accessor.get_data(gltf_data, binary) {
                AccessorData::Vec3(v) => v,
                _ => panic!("expected Vec3"),
            };

            let index_accessor = gltf_data.accessors.get(primitive.indices).unwrap();
            let indices = match index_accessor.get_data(gltf_data, binary) {
                accessor::AccessorData::Scalar(v) => v,
                _ => panic!("Expected scalars"),
            };

            indices.chunks(3).for_each(|tri| {
                let a = (&positions[tri[0] as usize]).into();
                let b = (&positions[tri[1] as usize]).into();
                let c = (&positions[tri[2] as usize]).into();

                let tri = Tri::new(a, b, c, None, None, 0); // TODO: Material
                children.push(tri);
            });
        }

        Mesh::new(children)
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        self.aabb.hit(ray, interval)
    }

    fn get_bounds(&self) -> &Bounds {
        self.aabb.get_bounds()
    }

    fn translate(&mut self, vec: &Vec3) {
        self.aabb.translate(vec);
    }
}
