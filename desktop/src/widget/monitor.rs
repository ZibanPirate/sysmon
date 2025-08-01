use anyhow::Result;
use common_types::{monitor::MonitorEvent, network::NetworkInfo};
use std::sync::{LazyLock, Mutex};
use tauri::{AppHandle, Emitter};

#[cfg(not(target_os = "windows"))]
use crate::_utils::bytes_to_string::bytes_to_string;
#[cfg(not(target_os = "windows"))]
use tauri::tray::TrayIcon;

#[cfg(target_os = "windows")]
use lib_cpp::get_network_info;
#[cfg(target_os = "macos")]
use lib_swift::get_network_info;

pub async fn start_monitoring(app_handle: AppHandle) {
    loop {
        if let Err(e) = run(&app_handle).await {
            // todo-zm: handle repeated error, show user a notification, with action to restart
            eprintln!("Error in monitor: {}", e);
        };
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}

trait NetworkSnapshot {
    fn refresh_and_get_monitor_event(&mut self, other: Self) -> (u64, u64);
}

impl NetworkSnapshot for NetworkInfo {
    fn refresh_and_get_monitor_event(&mut self, newer: Self) -> (u64, u64) {
        let result = (
            newer.total_sent - self.total_sent,
            newer.total_received - self.total_received,
        );

        *self = newer;

        result
    }
}

static NETWORK_SNAPSHOT: LazyLock<Mutex<NetworkInfo>> = LazyLock::new(|| {
    Mutex::new(NetworkInfo {
        total_sent: 0,
        total_received: 0,
    })
});

#[cfg(not(target_os = "windows"))]
static MAIN_TRAY: LazyLock<Mutex<Option<TrayIcon>>> = LazyLock::new(|| Mutex::new(None));

pub async fn run(app_handle: &AppHandle) -> Result<()> {
    #[cfg(target_os = "macos")]
    let current_network_info = get_network_info();
    #[cfg(target_os = "windows")]
    let current_network_info = get_network_info();

    let mut snapshot = NETWORK_SNAPSHOT.lock().unwrap();
    let network_speed = snapshot.refresh_and_get_monitor_event(current_network_info);

    app_handle.emit_to(
        "widget",
        "monitor_event",
        MonitorEvent::new_network_from_tuple(network_speed),
    )?;

    #[cfg(not(target_os = "windows"))]
    {
        let mut tray_lock = MAIN_TRAY.lock().unwrap();
        let tray = tray_lock.get_or_insert_with(|| {
            app_handle
                .tray_by_id("main")
                .expect("Tray with ID 'main' not found")
        });

        let chosen_speed = match network_speed.1 >= network_speed.0 {
            true => ("↓", network_speed.1),
            false => ("↑", network_speed.0),
        };

        tray.set_title(Some(&bytes_to_string(chosen_speed.1, chosen_speed.0)))?;
    }
    Ok(())
}
