use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum MonitorEvent {
    Network {
        #[typeshare(typescript(type = "number"))]
        sent: u64,
        #[typeshare(typescript(type = "number"))]
        received: u64,
    },
}

impl MonitorEvent {
    pub fn new_network_from_tuple((sent, received): (u64, u64)) -> Self {
        MonitorEvent::Network { sent, received }
    }
}
