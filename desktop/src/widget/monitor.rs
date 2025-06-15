use anyhow::Result;
use common_types::MonitorEvent;
use tauri::{AppHandle, Emitter};

pub async fn run(app_handle: AppHandle) -> Result<()> {
    let event = MonitorEvent { working: true };
    app_handle.emit_to("widget", "monitor_event", &event)?;
    Ok(())
}

pub async fn start_monitoring(app_handle: AppHandle) {
    loop {
        if let Err(e) = run(app_handle.clone()).await {
            // todo-zm: handle repeated error, show user a notification, with action to restart
            eprintln!("Error in monitor: {}", e);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
