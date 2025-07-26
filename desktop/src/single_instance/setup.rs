use anyhow::Result;

pub fn setup_single_instance<'a>(app: &'a mut tauri::App) -> Result<()> {
    app.handle()
        .plugin(tauri_plugin_single_instance::init(|_app, args, cwd| {
            // todo-zm: report-error
            println!(
                "Another instance tried to start with args: {:?}, cwd: {:?}",
                args, cwd
            );
        }))?;

    Ok(())
}
