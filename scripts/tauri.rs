#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
---

use cli_run::CliRun;

fn main() {
    let cmd = CliRun::new().with_relative_cwd("./desktop");
    let args = std::env::args().collect::<Vec<_>>();

    cmd.run(
        "npx",
        vec!["-y", "@tauri-apps/cli"]
            .into_iter()
            .chain(args[1..].iter().map(String::as_str))
            .collect::<Vec<_>>(),
    );
}
