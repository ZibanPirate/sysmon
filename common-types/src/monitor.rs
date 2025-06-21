use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum MonitorEvent {
    // CPU { percentage: f64 }, // todo-zm: implement CPU monitoring
    Network { sent: f64, received: f64 },
}

impl MonitorEvent {
    pub fn new_network_from_tuple(speed: (u64, u64)) -> Self {
        MonitorEvent::Network {
            sent: speed.0 as f64,
            received: speed.1 as f64,
        }
    }
}
