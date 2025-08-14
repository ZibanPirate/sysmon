#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
---

use cli_run::{cli_run, CliRun};
use std::io::Write;

fn main() {
    println!("- Installing ./desktop/web dependencies...");
    let cmd = CliRun::new().with_relative_cwd("./desktop/web");
    cmd.run("npm", vec!["install"]);

    println!("- Installing tauri-cli...");
    cli_run("cargo", vec!["install", "tauri-cli"]);

    println!("- Installing typeshare...");
    // todo-zm: switch to upstream once https://github.com/1Password/typeshare/pull/140 is merged
    cli_run(
        "cargo",
        vec![
            "install",
            "typeshare-cli",
            "--git",
            "https://github.com/tomjw64/typeshare.git",
            "--branch",
            "allow-override-for-disallowed-types",
        ],
    );

    #[cfg(target_os = "macos")]
    {
        println!("- macOS: Installing swift-bridge-cli");
        cli_run("cargo", vec!["install", "swift-bridge-cli"]);
    }

    println!("- Generating Tauri icons...");
    let cmd = CliRun::new().with_relative_cwd("./desktop");
    cmd.run("cargo", vec!["tauri", "icon", "./assets/svg/logo.svg"]);

    println!("- Generating .PS1 files...");
    let cwd = cli_run::get_cli_run_cwd();
    for entry in std::fs::read_dir(cwd.join("scripts")).unwrap() {
        let entry = entry.unwrap();
        if entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
            let ps1_path = entry.path().with_extension("rs.ps1");
            let mut file = std::fs::File::create(ps1_path).unwrap();
            writeln!(file, r#"$env:RUST_BACKTRACE = "1""#).unwrap();
            writeln!(
                file,
                r#"cargo +nightly -Zscript "{}" @args"#,
                entry.path().strip_prefix(&cwd).unwrap().to_string_lossy()
            )
            .unwrap();
        }
    }
}
