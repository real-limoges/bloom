pub const MAGIC: u32 = 0x424C4F4D;

pub const VERSION: u16 = 1;

pub const HEADER_SIZE: usize = 16;

#[repr(u16)]
pub enum Flags {
    None = 0,
    Compressed = 1 << 0,
    HasLabels = 1 << 1,
    HasWeights = 1 << 2,
}

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub magic: u32,
    pub version: u16,
    pub node_count: u32,
    pub edge_count: u32,
    pub flags: u16,
}

impl Header {
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < HEADER_SIZE {
            return Err(format!("Header is too short: {} bytes", bytes.len()));
        }

        let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if magic != MAGIC {
            return Err(format!("Invalid magic number: 0x{:08X}", magic));
        }

        let version = u16::from_le_bytes([bytes[4], bytes[5]]);
        if version != VERSION {
            return Err(format!("Unsupported version: {}", version));
        }

        Ok(Header {
            magic,
            version,
            node_count: u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]),
            edge_count: u32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]),
            flags: u16::from_le_bytes([bytes[14], bytes[15]]),
        })
    }

    pub fn has_flag(&self, flag: Flags) -> bool {
        (self.flags & flag as u16) != 0
    }
}
