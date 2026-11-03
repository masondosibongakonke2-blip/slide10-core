use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgePacket {
    pub id: String,
    pub priority: u8,
    pub anomalous: bool,
    pub payload_size: usize,
}

#[derive(Debug, Clone)]
pub struct EdgeFilter {
    pub min_priority: u8,
}

impl EdgeFilter {
    pub fn new(min_priority: u8) -> Self {
        Self { min_priority }
    }

    pub fn admit(&self, pkt: &EdgePacket) -> bool {
        if pkt.anomalous {
            return false;
        }
        pkt.priority >= self.min_priority
    }
      }
