use gltf::{AccessorData, GltfData, GltfMesh};
use util::{HitResult, Interval, Ray, Vec3};

use crate::{
    aabb::AABB,
    bounds::Bounds,
    hittable::{Hittable, HittableType},
    tri::Tri,
};

#[derive(Debug)]
pub struct Mesh {
    pub aabb: AABB,
}

impl Mesh {
    pub fn new(children: Vec<HittableType>) -> Self {
        let aabb = AABB::new(children);
        Mesh { aabb }
    }

    pub fn from_gltf_mesh(
        gltf_mesh: &GltfMesh,
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
            let AccessorData::Vec3(positions) = pos_accessor.get_data(gltf_data, binary) else {
                panic!("expected Vec3")
            };

            let normal_accessor = gltf_data
                .accessors
                .get(primitive.attributes.normal)
                .unwrap();
            let AccessorData::Vec3(normals) = normal_accessor.get_data(gltf_data, binary) else {
                panic!("expected Vec3")
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
            let AccessorData::Vec2(uvs) = uv_accessor.get_data(gltf_data, binary) else {
                panic!("expected Vec2")
            };

            let index_accessor = gltf_data.accessors.get(primitive.indices).unwrap();
            let indices: Vec<usize> = match index_accessor.get_data(gltf_data, binary) {
                AccessorData::Scalar(v) => v.into_iter().map(|i| i as usize).collect(),
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

                let na = Vec3::from(normals[tri[0]]).normalize();
                let nb = Vec3::from(normals[tri[1]]).normalize();
                let nc = Vec3::from(normals[tri[2]]).normalize();
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
                children.push(HittableType::Tri(tri));
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

    fn debug_hit_count(&self, ray: &Ray, interval: &Interval) -> u32 {
        self.aabb.debug_hit_count(ray, interval)
    }

    fn translate(&mut self, vec: &Vec3) {
        self.aabb.translate(vec);
    }

    fn scale(&mut self, vec: &Vec3) {
        self.aabb.scale(vec);
    }

    fn rotate(&mut self, axis: &Vec3, angle_rad: f32) {
        self.aabb.rotate(axis, angle_rad);
    }
}
