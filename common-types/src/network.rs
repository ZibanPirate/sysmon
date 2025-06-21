#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub total_sent: u32,
    pub total_received: u32,
}
impl NetworkInfo {
    pub fn new(total_sent: u32, total_received: u32) -> NetworkInfo {
        NetworkInfo {
            total_sent,
            total_received,
        }
    }
}
