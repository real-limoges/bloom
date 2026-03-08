use super::format::{HEADER_SIZE, Header};
use crate::graph::types::{Edge, Graph, Node};

pub struct Decoder<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn decode_graph(&mut self) -> Result<Graph, String> {
        let header = Header::parse(self.data)?;
        self.offset = HEADER_SIZE;

        let labels = if header.has_flag(super::format::Flags::HasLabels) {
            self.decode_string_table(header.node_count as usize)?
        } else {
            vec![String::new(); header.node_count as usize]
        };

        let (ids, pageranks, degrees) = self.decode_node_data(header.node_count as usize)?;
        let (sources, targets) = self.decode_edge_data(header.edge_count as usize)?;

        let nodes = ids
            .into_iter()
            .zip(labels)
            .zip(pageranks)
            .zip(degrees)
            .map(|(((id, label), pagerank), degree)| Node {
                id,
                label,
                pagerank,
                degree,
                x: 0.0,
                y: 0.0,
            })
            .collect();

        let edges = sources
            .into_iter()
            .zip(targets)
            .map(|(source, target)| Edge { source, target })
            .collect();

        Ok(Graph::new(nodes, edges))
    }

    fn decode_string_table(&mut self, count: usize) -> Result<Vec<String>, String> {
        let total_len = self.read_u32()? as usize;
        let offsets: Vec<u32> = (0..count)
            .map(|_| self.read_u32())
            .collect::<Result<_, _>>()?;

        let string_data = self.read_bytes(total_len)?;

        let mut labels = Vec::with_capacity(count);
        for i in 0..count {
            let start = offsets[i] as usize;
            let end = if i + 1 < count {
                offsets[i + 1] as usize
            } else {
                total_len
            };
            let s = std::str::from_utf8(&string_data[start..end])
                .map_err(|e| format!("Invalid UTF-8: {}", e))?;
            labels.push(s.to_string());
        }
        Ok(labels)
    }

    #[allow(clippy::type_complexity)]
    fn decode_node_data(&mut self, count: usize) -> Result<(Vec<u32>, Vec<f32>, Vec<u16>), String> {
        let ids = self.read_u32_array(count)?;
        let pageranks = self.read_f32_array(count)?;
        let degrees = self.read_u16_array(count)?;
        Ok((ids, pageranks, degrees))
    }

    fn decode_edge_data(&mut self, count: usize) -> Result<(Vec<u32>, Vec<u32>), String> {
        let sources = self.read_u32_array(count)?;
        let targets = self.read_u32_array(count)?;
        Ok((sources, targets))
    }

    // primatives

    fn read_u32_array(&mut self, count: usize) -> Result<Vec<u32>, String> {
        (0..count).map(|_| self.read_u32()).collect()
    }

    fn read_u16_array(&mut self, count: usize) -> Result<Vec<u16>, String> {
        (0..count).map(|_| self.read_u16()).collect()
    }

    fn read_f32_array(&mut self, count: usize) -> Result<Vec<f32>, String> {
        (0..count).map(|_| self.read_f32()).collect()
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        let bytes = self.read_bytes(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn read_u16(&mut self) -> Result<u16, String> {
        let bytes = self.read_bytes(2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn read_f32(&mut self) -> Result<f32, String> {
        let bytes = self.read_bytes(4)?;
        Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn read_bytes(&mut self, len: usize) -> Result<&[u8], String> {
        if self.offset + len > self.data.len() {
            return Err(format!("Unexpected EOF at offset {}", self.offset));
        }
        let slice = &self.data[self.offset..self.offset + len];
        self.offset += len;
        Ok(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::format::{Flags, HEADER_SIZE, MAGIC, VERSION};

    fn build_blom(
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

    #[test]
    fn decode_minimal_graph() {
        let data = build_blom(&[], &[], None);
        let graph = Decoder::new(&data).decode_graph().unwrap();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn decode_nodes_no_labels() {
        let nodes = &[(1, 0.5f32, 3u16), (2, 0.25, 1)];
        let data = build_blom(nodes, &[], None);
        let graph = Decoder::new(&data).decode_graph().unwrap();

        assert_eq!(graph.node_count(), 2);
        let n0 = &graph.nodes()[0];
        assert_eq!(n0.id, 1);
        assert_eq!(n0.pagerank, 0.5);
        assert_eq!(n0.degree, 3);
        assert!(n0.label.is_empty());
        assert_eq!(n0.x, 0.0);
        assert_eq!(n0.y, 0.0);

        let n1 = &graph.nodes()[1];
        assert_eq!(n1.id, 2);
        assert_eq!(n1.pagerank, 0.25);
        assert_eq!(n1.degree, 1);
    }

    #[test]
    fn decode_edges() {
        let nodes = &[(10, 0.0, 1), (20, 0.0, 1)];
        let edges = &[(10u32, 20u32)];
        let data = build_blom(nodes, edges, None);
        let graph = Decoder::new(&data).decode_graph().unwrap();

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.edges()[0].source, 10);
        assert_eq!(graph.edges()[0].target, 20);
    }

    #[test]
    fn decode_with_labels() {
        let nodes = &[(1, 0.1, 0), (2, 0.2, 0)];
        let labels = &["hello", "world"];
        let data = build_blom(nodes, &[], Some(labels));
        let graph = Decoder::new(&data).decode_graph().unwrap();

        assert_eq!(graph.nodes()[0].label, "hello");
        assert_eq!(graph.nodes()[1].label, "world");
    }

    #[test]
    fn decode_truncated_data() {
        let mut data = build_blom(&[(1, 0.0, 0)], &[], None);
        data.truncate(HEADER_SIZE + 2); // cut off mid-node-data
        let err = Decoder::new(&data).decode_graph().unwrap_err();
        assert!(err.contains("Unexpected EOF"), "got: {err}");
    }

    #[test]
    fn decode_roundtrip_counts() {
        let nodes = &[(1, 0.1, 2), (2, 0.2, 3), (3, 0.3, 1)];
        let edges = &[(1, 2), (2, 3)];
        let data = build_blom(nodes, edges, None);
        let graph = Decoder::new(&data).decode_graph().unwrap();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn decode_node_index_lookup() {
        let nodes = &[(42, 0.0, 0), (99, 0.0, 0)];
        let data = build_blom(nodes, &[], None);
        let graph = Decoder::new(&data).decode_graph().unwrap();

        assert_eq!(graph.node_by_id(42).unwrap().id, 42);
        assert_eq!(graph.node_by_id(99).unwrap().id, 99);
        assert!(graph.node_by_id(1).is_none());
    }
}
