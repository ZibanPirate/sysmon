use serde::Serialize;
use sysinfo::Networks;
use tauri::Manager;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct NetworkSpeed {
    received_bytes: u64,
    sent_bytes: u64,
    utc_timestamp: u64,
}

impl NetworkSpeed {
    fn new() -> NetworkSpeed {
        NetworkSpeed {
            received_bytes: 0,
            sent_bytes: 0,
            utc_timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    fn update(&mut self, received_bytes: u64, sent_bytes: u64) {
        self.received_bytes = received_bytes;
        self.sent_bytes = sent_bytes;
        self.utc_timestamp = chrono::Utc::now().timestamp_millis() as u64;
    }
}

fn bytes_to_string(bytes: u64, suffix: &str) -> String {
    format!(
        "{} {}",
        match bytes {
            0..=1023 => "00.00 K".to_string(),
            1024..=1_048_575 => format!("{:0>5.2} K", bytes as f64 / 1024.0),
            1_048_576..=1_073_741_824 => format!("{:0>5.2} M", bytes as f64 / 1024.0 / 1024.0),
            _ => format!("{:0>5.2} G", bytes as f64 / 1024.0 / 1024.0 / 1024.0),
        },
        suffix
    )
}

async fn monitor_system(target_window: tauri::Window) {
    let mut speed = NetworkSpeed::new();

    let mut networks = Networks::new_with_refreshed_list();

    networks.refresh();
    loop {
        let mut bytes_received = 0;
        let mut bytes_sent = 0;
        for (_, network) in &networks {
            bytes_received += network.received();
            bytes_sent += network.transmitted();
        }
        networks.refresh();
        speed.update(bytes_received, bytes_sent);
        if target_window.is_visible().unwrap() {
            let emitting_result = target_window.emit("network-info", speed.clone());
            match emitting_result {
                Ok(_) => (),
                Err(e) => println!("Error emitting: {}", e),
            }
        }

        let chosen_speed = match speed.received_bytes >= speed.sent_bytes {
            true => ("↓", speed.received_bytes),
            false => ("↑", speed.sent_bytes),
        };

        target_window
            .app_handle()
            .tray()
            .unwrap()
            .set_title(Some(&bytes_to_string(chosen_speed.1, chosen_speed.0)))
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}

pub fn register_monitor_for_window(target_window: tauri::Window) {
    tokio::spawn(monitor_system(target_window));
}
