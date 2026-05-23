use std::io::Read;

use crate::{
    geometry::tri::Tri,
    material::material_trait::MaterialType,
    util::parser::glb::{
        gltf::GltfData,
        types::{Chunk, ChunkType},
    },
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

    let mut chunks = vec![];
    let mut offset = 12;
    while offset < buffer.len() {
        let chunk = parse_chunk(&buffer, offset);
        offset += 8 + chunk.length as usize;
        chunks.push(chunk);
    }

    // First chunk should be JSON GltfData, second chunk should be binary
    let gltf_data = {
        let json_chunk = chunks
            .first()
            .expect("GLB file must contain at least one chunk");
        let json_str = String::from_utf8_lossy(&json_chunk.data);
        serde_json::from_str::<GltfData>(&json_str).expect("Failed to parse JSON chunk as GltfData")
    };

    dbg!("Parsed GLTF data", gltf_data);

    (vec![], vec![])
}

fn parse_chunk(buffer: &[u8], offset: usize) -> Chunk {
    let length = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
    let chunk_type = u32::from_le_bytes(buffer[offset + 4..offset + 8].try_into().unwrap()).into();
    let data = buffer[offset + 8..offset + 8 + length as usize].to_vec();

    match chunk_type {
        ChunkType::Json => {
            let json_str = String::from_utf8_lossy(&data);
            println!("Found JSON chunk with length {length} bytes");
            println!("JSON content: {json_str}");
        }
        ChunkType::Binary => println!("Found Binary chunk with length {length} bytes"),
    }

    Chunk {
        length,
        chunk_type,
        data,
    }
}
