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
