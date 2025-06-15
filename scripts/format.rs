#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
glob = "0.3"
---

use glob::glob;
use std::path::PathBuf;
use std::time::Duration;

fn main() {
    let cwd = cli_run::get_cli_run_cwd();

    println!("- Formatting script files...");
    for entry in
        glob(cwd.join("./scripts/**/*.rs").to_str().unwrap()).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                let (changed, duration) = format_dot_rs_script_file(&path);
                println!(
                    "{} {}ms {}",
                    path.strip_prefix(&cwd).unwrap_or(&path).display(),
                    duration.as_millis(),
                    if changed { "" } else { "(unchanged)" },
                );
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("- Formatting Rust workspace ...");
    cli_run::cli_run("cargo", vec!["fmt"]);

    println!("- Formatting everything else with Prettier ...");
    // todo:zm replace with https://biomejs.dev/internals/language-support or https://github.com/oxc-project
    cli_run::cli_run("npx", vec!["-y", "prettier", "--write", "."]);
}

fn format_dot_rs_script_file(file_path: &PathBuf) -> (bool, Duration) {
    let start = std::time::Instant::now();

    let content = std::fs::read_to_string(file_path).unwrap();
    let mut splits = content.split("---\n");
    let before_cargo_toml = splits.next().unwrap().trim().to_string();
    let cargo_toml_content = splits.next().unwrap().trim().to_string();
    let after_cargo_toml = splits.next().unwrap().trim().to_string();
    let file_without_cargo_toml = format!("{}\n{}\n", before_cargo_toml, after_cargo_toml);
    std::fs::write(file_path, file_without_cargo_toml).unwrap();

    cli_run::cli_run("rustfmt", vec![file_path.display().to_string()]);
    let mut formatted_file = std::fs::read_to_string(file_path).unwrap();
    let index_of_first_newline = formatted_file.find('\n').unwrap();
    let cargo_toml_content_with_borders = format!("\n---\n{}\n---\n\n", cargo_toml_content);
    formatted_file.insert_str(index_of_first_newline + 1, &cargo_toml_content_with_borders);
    std::fs::write(file_path, &formatted_file).unwrap();

    let duration = start.elapsed();
    (content != formatted_file, duration)
}
