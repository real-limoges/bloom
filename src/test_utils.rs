use crate::protocol::format::{Flags, HEADER_SIZE, MAGIC, VERSION};

pub fn build_blom(
    nodes: &[(u32, f32, u16)],
    edges: &[(u32, u32)],
    labels: Option<&[&str]>,
) -> Vec<u8> {
    let flags = if labels.is_some() {
        Flags::HasLabels as u16
    } else {
        0
    };
    let mut buf = Vec::new();

    // Header
    buf.extend_from_slice(&MAGIC.to_le_bytes());
    buf.extend_from_slice(&VERSION.to_le_bytes());
    buf.extend_from_slice(&(nodes.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(edges.len() as u32).to_le_bytes());
    buf.extend_from_slice(&flags.to_le_bytes());
    assert_eq!(buf.len(), HEADER_SIZE);

    // String table (if labels present)
    if let Some(labels) = labels {
        let concat: Vec<u8> = labels.iter().flat_map(|s| s.as_bytes()).copied().collect();
        let total_len = concat.len() as u32;
        buf.extend_from_slice(&total_len.to_le_bytes());

        let mut offset = 0u32;
        for s in labels {
            buf.extend_from_slice(&offset.to_le_bytes());
            offset += s.len() as u32;
        }
        buf.extend_from_slice(&concat);
    }

    // Node data: ids, pageranks, degrees
    for &(id, _, _) in nodes {
        buf.extend_from_slice(&id.to_le_bytes());
    }
    for &(_, pr, _) in nodes {
        buf.extend_from_slice(&pr.to_le_bytes());
    }
    for &(_, _, deg) in nodes {
        buf.extend_from_slice(&deg.to_le_bytes());
    }

    // Edge data: sources, targets
    for &(src, _) in edges {
        buf.extend_from_slice(&src.to_le_bytes());
    }
    for &(_, tgt) in edges {
        buf.extend_from_slice(&tgt.to_le_bytes());
    }

    buf
}
