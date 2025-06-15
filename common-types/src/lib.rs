use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum MonitorEvent {
    // CPU { percentage: f64 }, // todo-zm: implement CPU monitoring
    Network { sent: f64, received: f64 },
}
