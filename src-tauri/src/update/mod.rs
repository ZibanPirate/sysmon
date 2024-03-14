use tauri_plugin_updater::Updater;

async fn check_and_update(updater: Updater) {
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

pub fn register_updater(updater: Updater) {
    tokio::spawn(check_and_update(updater));
}
