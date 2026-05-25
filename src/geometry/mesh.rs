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

    pub fn from_gltf_mesh(
        gltf_mesh: &gltf::Mesh,
        gltf_data: &GltfData,
        binary: &[u8],
        mat_offset: usize,
    ) -> Self {
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

            let normal_accessor = gltf_data
                .accessors
                .get(primitive.attributes.normal)
                .unwrap();
            let normals = match normal_accessor.get_data(gltf_data, binary) {
                AccessorData::Vec3(v) => v,
                _ => panic!("expected Vec3"),
            };

            let tangents = match primitive.attributes.tangent {
                Some(tan_index) => {
                    let tangent_accessor = gltf_data.accessors.get(tan_index).unwrap();
                    let tangents: Vec<[f32; 4]> = match tangent_accessor.get_data(gltf_data, binary)
                    {
                        AccessorData::Vec4(v) => {
                            v.into_iter().map(|i| i.map(|z| z as f32)).collect()
                        }
                        _ => panic!("expected Vec4"),
                    };

                    Some(tangents)
                }
                None => None,
            };

            let uv_accessor = gltf_data
                .accessors
                .get(primitive.attributes.texcoord_0)
                .unwrap();
            let uvs = match uv_accessor.get_data(gltf_data, binary) {
                AccessorData::Vec2(v) => v,
                _ => panic!("expected Vec3"),
            };

            let index_accessor = gltf_data.accessors.get(primitive.indices).unwrap();
            let indices: Vec<usize> = match index_accessor.get_data(gltf_data, binary) {
                accessor::AccessorData::Scalar(v) => v.into_iter().map(|i| i as usize).collect(),
                _ => panic!("Expected scalars"),
            };

            indices.chunks(3).for_each(|tri| {
                let a = positions[tri[0]].into();
                let b = positions[tri[1]].into();
                let c = positions[tri[2]].into();

                let uva = uvs[tri[0]].into();
                let uvb = uvs[tri[1]].into();
                let uvc = uvs[tri[2]].into();
                let uvs = Some((uva, uvb, uvc));

                let na = normals[tri[0]].into();
                let nb = normals[tri[1]].into();
                let nc = normals[tri[2]].into();
                let normals = Some((na, nb, nc));

                let tan = match &tangents {
                    Some(t) => {
                        let ta = t[tri[0]];
                        let tb = t[tri[1]];
                        let tc = t[tri[2]];
                        Some([ta, tb, tc])
                    }
                    None => None,
                };

                let tri = Tri::new(
                    a,
                    b,
                    c,
                    normals,
                    uvs,
                    tan,
                    primitive.material.map(|m| m + mat_offset),
                );
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

    fn scale(&mut self, vec: &Vec3) {
        self.aabb.scale(vec);
    }
}
