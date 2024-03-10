use serde::Serialize;
use tauri::Manager;

#[derive(Debug, Serialize, Clone)]
struct NetworkSpeed {
    packets_per_second: u32,
    utc_timestamp: u64,
}

impl NetworkSpeed {
    fn new() -> NetworkSpeed {
        NetworkSpeed {
            packets_per_second: 0,
            utc_timestamp: 0,
        }
    }

    fn update(&mut self, packets_per_second: u32) {
        self.packets_per_second = packets_per_second;
        self.utc_timestamp = chrono::Utc::now().timestamp() as u64;
    }
}

async fn monitor_system(target_window: tauri::Window) {
    let mut speed = NetworkSpeed::new();
    let mut index = 0;
    loop {
        speed.update(index);
        let emitting_result = target_window.emit("network-info", speed.clone());
        match emitting_result {
            Ok(_) => (),
            Err(e) => println!("Error emitting: {}", e),
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        index += 1;
    }
}

pub fn register_monitor_for_window(target_window: tauri::Window) {
    tokio::spawn(monitor_system(target_window));
}
