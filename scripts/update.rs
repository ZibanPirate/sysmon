#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
---

use cli_run::CliRun;

fn main() {
    println!("- Updating Rust dependencies ...");
    cli_run::cli_run("cargo", vec!["update"]);

    println!("- Updating ./desktop/web dependencies ...");
    let cmd = CliRun::new().with_relative_cwd("./desktop/web");
    cmd.run("npm", vec!["update"]);
}
