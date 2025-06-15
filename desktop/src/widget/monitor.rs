use anyhow::Result;
use common_types::MonitorEvent;
use sysinfo::Networks;
use tauri::{AppHandle, Emitter};

pub async fn run(app_handle: &AppHandle, networks: &mut Networks) -> Result<()> {
    let mut all_interfaces_data = (0.0f64, 0.0f64);
    for (_, data) in networks.into_iter() {
        all_interfaces_data.0 += data.received() as f64;
        all_interfaces_data.1 += data.transmitted() as f64;
    }
    networks.refresh(true);

    let event = MonitorEvent::Network {
        sent: all_interfaces_data.1,
        received: all_interfaces_data.0,
    };
    app_handle.emit_to("widget", "monitor_event", &event)?;
    Ok(())
}

pub async fn start_monitoring(app_handle: AppHandle) {
    let mut networks = Networks::new_with_refreshed_list();
    networks.refresh(true);

    loop {
        if let Err(e) = run(&app_handle, &mut networks).await {
            // todo-zm: handle repeated error, show user a notification, with action to restart
            eprintln!("Error in monitor: {}", e);
        };
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}
