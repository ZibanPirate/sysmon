use anyhow::Result;
use common_types::{MonitorEvent, NetworkInfo};
use lib_swift::get_network_info;
use std::sync::{LazyLock, Mutex};
use tauri::{AppHandle, Emitter};

trait NetworkSnapshot {
    fn refresh_and_get_monitor_event(&mut self, other: Self) -> MonitorEvent;
}

impl NetworkSnapshot for NetworkInfo {
    fn refresh_and_get_monitor_event(&mut self, newer: Self) -> MonitorEvent {
        let event = MonitorEvent::Network {
            sent: newer.total_sent as f64 - self.total_sent as f64,
            received: newer.total_received as f64 - self.total_received as f64,
        };

        *self = newer;

        event
    }
}

static NETWORK_SNAPSHOT: LazyLock<Mutex<NetworkInfo>> = LazyLock::new(|| {
    Mutex::new(NetworkInfo {
        total_sent: 0,
        total_received: 0,
    })
});

pub async fn run(app_handle: &AppHandle) -> Result<()> {
    let current_network_info = get_network_info();
    let mut snapshot = NETWORK_SNAPSHOT.lock().unwrap();
    let network_event = snapshot.refresh_and_get_monitor_event(current_network_info);

    app_handle.emit_to("widget", "monitor_event", &network_event)?;
    Ok(())
}

pub async fn start_monitoring(app_handle: AppHandle) {
    loop {
        if let Err(e) = run(&app_handle).await {
            // todo-zm: handle repeated error, show user a notification, with action to restart
            eprintln!("Error in monitor: {}", e);
        };
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}
