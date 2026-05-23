pub struct GlbHeader {
    magic: [u8; 4],
    version: u32,
    length: u32,
}

pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
}

pub enum ChunkType {
    Json = 0x4E4F534A,   // ASCII "JSON"
    Binary = 0x004E4942, // ASCII "BIN\0"
}

impl From<u32> for ChunkType {
    fn from(value: u32) -> Self {
        match value {
            0x4E4F534A => ChunkType::Json,
            0x004E4942 => ChunkType::Binary,
            _ => panic!("Unknown chunk type: {value:#X}"),
        }
    }
}
