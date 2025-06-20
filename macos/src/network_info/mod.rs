use common_types::NetworkInfo;
use std::time::Instant;
use swift_rs::SRObject;
use swift_rs::swift;

#[repr(C)]
#[derive(Debug, Clone)]
struct CNetworkInfo {
    total_sent: isize,
    total_received: isize,
}

impl Into<NetworkInfo> for &CNetworkInfo {
    fn into(self) -> NetworkInfo {
        NetworkInfo {
            total_sent: self.total_sent,
            total_received: self.total_received,
            timestamp: Instant::now(),
        }
    }
}

swift!(fn network_info() -> SRObject<CNetworkInfo>);

pub fn get_network_info() -> NetworkInfo {
    let result = unsafe { network_info() };
    result.as_ref().into()
}

// todo-zm: add tests
