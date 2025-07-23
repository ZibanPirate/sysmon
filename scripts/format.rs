#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
glob = "0.3"
---

use glob::glob;

fn main() {
    let cwd = cli_run::get_cli_run_cwd();

    // todo-zm: remove this once https://github.com/rust-lang/style-team/issues/212 is resolved
    println!("- Formatting script files...");
    let script_file_paths = glob(&format!("{}/scripts/*.rs", cwd.display()))
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .map(|path| path.to_string_lossy().to_string())
        .collect::<Vec<String>>();
    cli_run::cli_run("rustfmt", script_file_paths);

    println!("- Formatting Rust workspace ...");
    cli_run::cli_run("cargo", vec!["fmt"]);

    println!("- Formatting everything else with Prettier ...");
    // todo:zm replace with https://biomejs.dev/internals/language-support or https://github.com/oxc-project
    cli_run::cli_run("npx", vec!["-y", "prettier", "--write", "."]);
}
