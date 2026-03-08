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

#[cfg(test)]
mod tests {
    use super::*;

    fn build_header(node_count: u32, edge_count: u32, flags: u16) -> Vec<u8> {
        let mut buf = Vec::with_capacity(HEADER_SIZE);
        buf.extend_from_slice(&MAGIC.to_le_bytes());
        buf.extend_from_slice(&VERSION.to_le_bytes());
        buf.extend_from_slice(&node_count.to_le_bytes());
        buf.extend_from_slice(&edge_count.to_le_bytes());
        buf.extend_from_slice(&flags.to_le_bytes());
        buf
    }

    #[test]
    fn parse_valid_header() {
        let data = build_header(100, 50, Flags::HasLabels as u16);
        let h = Header::parse(&data).unwrap();
        assert_eq!(h.magic, MAGIC);
        assert_eq!(h.version, VERSION);
        assert_eq!(h.node_count, 100);
        assert_eq!(h.edge_count, 50);
        assert_eq!(h.flags, Flags::HasLabels as u16);
    }

    #[test]
    fn parse_too_short() {
        let err = Header::parse(&[0u8; 10]).unwrap_err();
        assert!(err.contains("too short"), "got: {err}");
    }

    #[test]
    fn parse_bad_magic() {
        let mut data = build_header(0, 0, 0);
        data[0..4].copy_from_slice(&0xDEADBEEFu32.to_le_bytes());
        let err = Header::parse(&data).unwrap_err();
        assert!(err.contains("Invalid magic"), "got: {err}");
    }

    #[test]
    fn parse_bad_version() {
        let mut data = build_header(0, 0, 0);
        data[4..6].copy_from_slice(&99u16.to_le_bytes());
        let err = Header::parse(&data).unwrap_err();
        assert!(err.contains("Unsupported version"), "got: {err}");
    }

    #[test]
    fn has_flag_none() {
        let h = Header::parse(&build_header(0, 0, 0)).unwrap();
        assert!(!h.has_flag(Flags::Compressed));
        assert!(!h.has_flag(Flags::HasLabels));
        assert!(!h.has_flag(Flags::HasWeights));
    }

    #[test]
    fn has_flag_single() {
        let h = Header::parse(&build_header(0, 0, Flags::HasLabels as u16)).unwrap();
        assert!(h.has_flag(Flags::HasLabels));
        assert!(!h.has_flag(Flags::Compressed));
        assert!(!h.has_flag(Flags::HasWeights));
    }

    #[test]
    fn has_flag_multiple() {
        let flags = Flags::Compressed as u16 | Flags::HasLabels as u16;
        let h = Header::parse(&build_header(0, 0, flags)).unwrap();
        assert!(h.has_flag(Flags::Compressed));
        assert!(h.has_flag(Flags::HasLabels));
        assert!(!h.has_flag(Flags::HasWeights));
    }

    #[test]
    fn magic_constant() {
        assert_eq!(MAGIC, 0x424C4F4D);
    }
}
