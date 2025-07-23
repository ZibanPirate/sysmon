use tauri::App;
use tauri_plugin_updater::{Updater, UpdaterExt};

// todo-zm: report-error
async fn check_and_download_and_wait_for_restart(updater: Updater) {
    println!("Checking for updates");
    let check_result = updater.check().await;
    match check_result {
        Ok(update) => match update {
            Some(update) => {
                match update
                    .download_and_install(
                        |uz, u64| {
                            println!("{} of {:?} bytes", uz, u64);
                        },
                        || {
                            println!("Download complete");
                        },
                    )
                    .await
                {
                    Ok(_) => {
                        println!("Update installed, restarting app");
                    }
                    Err(e) => {
                        println!("Error installing update: {:?}", e);
                    }
                }
            }
            None => {
                println!("No updates available");
            }
        },
        Err(e) => {
            println!("Error checking for updates: {:?}", e);
        }
    }
}

pub fn setup_updater(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        println!("Skipping updater setup because debug build");
        return Ok(());
    }

    app.handle()
        .plugin(tauri_plugin_updater::Builder::new().build())?;

    let updater = app.updater()?;

    tokio::spawn(check_and_download_and_wait_for_restart(updater));

    Ok(())
}
