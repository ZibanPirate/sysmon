#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub total_sent: u64,
    pub total_received: u64,
}

impl NetworkInfo {
    pub fn new(total_sent: u64, total_received: u64) -> NetworkInfo {
        NetworkInfo {
            total_sent,
            total_received,
        }
    }
}
