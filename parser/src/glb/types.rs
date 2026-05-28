pub struct GlbHeader {
    pub version: u32,
    pub length: u32,
}

impl From<&[u8; 12]> for GlbHeader {
    fn from(buffer: &[u8; 12]) -> Self {
        let preamble = &buffer[0..12];
        assert!(
            preamble[0..4] == [0x67, 0x6C, 0x54, 0x46],
            "Invalid GLB file: missing 'glTF' magic header"
        );

        let version = u32::from_le_bytes(preamble[4..8].try_into().unwrap());
        let length = u32::from_le_bytes(preamble[8..12].try_into().unwrap());

        Self { version, length }
    }
}

pub struct Chunk {
    pub length: u32,
    pub r#type: ChunkType,
    pub data: Vec<u8>,
}

pub enum ChunkType {
    Json = 0x4E4F_534A,   // ASCII "JSON"
    Binary = 0x004E_4942, // ASCII "BIN\0"
}

impl From<u32> for ChunkType {
    fn from(value: u32) -> Self {
        match value {
            0x4E4F_534A => ChunkType::Json,
            0x004E_4942 => ChunkType::Binary,
            _ => panic!("Unknown chunk type: {value:#X}"),
        }
    }
}
