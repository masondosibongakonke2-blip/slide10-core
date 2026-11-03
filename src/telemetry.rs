use serde::{Deserialize, Serialize};
use crate::mode::NetworkMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTelemetry {
    pub distance_m: f64,
    pub fade_margin_db: f64,
    pub packet_loss: f64,
    pub latency_ms: f64,
}

impl LinkTelemetry {
    pub fn suggest_mode(&self) -> NetworkMode {
        if self.packet_loss >= 0.45 || self.fade_margin_db < 3.0 || self.latency_ms > 2500.0 {
            NetworkMode::Extreme
        } else if self.packet_loss >= 0.20 || self.fade_margin_db < 8.0 || self.latency_ms > 800.0 {
            NetworkMode::Partitioned
        } else {
            NetworkMode::Normal
        }
    }
          }
