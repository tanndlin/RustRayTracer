use std::io::Read;

use crate::{
    geometry::tri::Tri, material::material_trait::MaterialType, util::parser::glb::types::Chunk,
};

pub fn parse_glb(path: &str) -> (Vec<Tri>, Vec<MaterialType>) {
    let mut buffer = vec![];
    std::fs::File::open(path)
        .expect("Failed to open .glb file")
        .read_to_end(&mut buffer)
        .expect("Failed to read .glb file");

    let preamble = &buffer[0..12];
    if preamble[0..4] != [0x67, 0x6C, 0x54, 0x46] {
        panic!("Invalid GLB file: missing 'glTF' magic header");
    }

    let version = u32::from_le_bytes(preamble[4..8].try_into().unwrap());
    if version != 2 {
        panic!("Unsupported GLB version: {}", version);
    }

    let length = u32::from_le_bytes(preamble[8..12].try_into().unwrap());
    println!("Paring GLB version {version} file {path}. Size: {length} bytes");

    return (vec![], vec![]);
}

fn parse_chunk(buffer: &[u8], offset: usize) -> Chunk {
    todo!()
}
