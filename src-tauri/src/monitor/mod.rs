use serde::Serialize;
use sysinfo::Networks;
use tauri::Manager;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct NetworkSpeed {
    packets: u64,
    utc_timestamp: u64,
}

impl NetworkSpeed {
    fn new() -> NetworkSpeed {
        NetworkSpeed {
            packets: 0,
            utc_timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    fn update(&mut self, packets_per_second: u64) {
        self.packets = packets_per_second;
        self.utc_timestamp = chrono::Utc::now().timestamp_millis() as u64;
    }
}

async fn monitor_system(target_window: tauri::Window) {
    let mut speed = NetworkSpeed::new();

    let mut networks = Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        println!("[{interface_name}]: {network:?}");
    }

    networks.refresh();
    loop {
        let mut bytes_received = 0;
        for (_, network) in &networks {
            bytes_received += network.received();
        }
        networks.refresh();
        speed.update(bytes_received);
        let emitting_result = target_window.emit("network-info", speed.clone());
        match emitting_result {
            Ok(_) => (),
            Err(e) => println!("Error emitting: {}", e),
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}

pub fn register_monitor_for_window(target_window: tauri::Window) {
    tokio::spawn(monitor_system(target_window));
}
