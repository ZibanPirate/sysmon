#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
---

use cli_run::{cli_run, CliRun};

// todo-zm: make this work on PowerShell

fn main() {
    println!("- Installing ./desktop/web dependencies...");
    let cmd = CliRun::new().with_relative_cwd("./desktop/web");
    cmd.run("npm", vec!["install"]);

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

    println!("- Generating Tauri icons...");
    let cmd = CliRun::new().with_relative_cwd("./desktop");
    cmd.run(
        "npx",
        vec!["-y", "@tauri-apps/cli", "icon", "./assets/svg/logo.svg"],
    );
}
