// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalPosition, LogicalSize, Manager, WindowBuilder};

async fn monitor_system(target_window: tauri::Window) {
    let mut index = 0;
    loop {
        println!("Monitoring system");
        let emitting_result = target_window.emit("network-info", Some(format!("{}", index)));
        match emitting_result {
            Ok(_) => println!("Emitted successfully"),
            Err(e) => println!("Error emitting: {}", e),
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        index += 1;
    }
}

const POSITION: LogicalPosition<f64> = LogicalPosition::new(0.0, 0.0);
const SIZE: LogicalSize<f64> = LogicalSize::new(400.0, 400.0);
const TITTLE: &str = "System Monitor";
const LABEL: &str = "main";

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[allow(unused_mut)]
            let mut window_builder = WindowBuilder::new(app, LABEL);
            #[cfg(target_os = "macos")]
            {
                window_builder = window_builder.tabbing_identifier(LABEL);
            }
            let settings_window = window_builder
                .title(TITTLE)
                .inner_size(SIZE.width, SIZE.height)
                .build()?;

            let webview_builder =
                tauri::WebviewBuilder::new(LABEL, tauri::WebviewUrl::App("index.html".into()));

            settings_window.add_child(webview_builder, POSITION, SIZE)?;
            settings_window.get_webview(LABEL).unwrap().open_devtools();
            // .emit("click", Some("Hello from Rust!"));
            tokio::spawn(monitor_system(settings_window));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run tauri application");
}
