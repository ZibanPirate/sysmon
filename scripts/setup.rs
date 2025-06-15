#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
---

use cli_run::CliRun;

fn main() {
    println!("- Installing ./desktop/web dependencies...");
    let cmd = CliRun::new().with_relative_cwd("./desktop/web");
    cmd.run("npm", vec!["install"]);

    println!("- Generating Tauri icons...");
    let cmd = CliRun::new().with_relative_cwd("./desktop");
    cmd.run(
        "npx",
        vec!["-y", "@tauri-apps/cli", "icon", "./assets/svg/logo.svg"],
    );
}
