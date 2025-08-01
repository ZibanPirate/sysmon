#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
clap = {version="4", features=["derive"]}
serde = { version = "1", features = ["derive"] }
toml = "0.9"
nest_struct = "0.5"
semver = "1"
regex = "1"
---

use clap::Parser;
use cli_run::CliRun;
use nest_struct::nest_struct;
use regex::Regex;
use semver::Version;
use serde::{Deserialize, Serialize};

#[nest_struct]
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[derive(PartialEq)]
struct Args {
    #[clap(long)]
    commit: bool,

    #[clap(subcommand)]
    command: nest! {
        Patch,
        Minor,
        Major,
        InferFromConventionalCommits,
        UpdateReadmeWithCurrentVersion,
    },
}

impl ArgsCommand {
    fn bump(&self, version: &Version) -> Version {
        let mut version = version.clone();
        match self {
            ArgsCommand::Patch => {
                version.patch += 1;
            }
            ArgsCommand::Minor => {
                if version.major < 1 {
                    version.patch += 1;
                } else {
                    version.minor += 1;
                    version.patch = 0;
                }
            }
            ArgsCommand::Major => {
                if version.major < 1 {
                    version.minor += 1;
                } else {
                    version.major += 1;
                    version.minor = 0;
                }
                version.patch = 0;
            }
            ArgsCommand::InferFromConventionalCommits => {
                // todo-zm: Implement logic to infer version bump from conventional commits
                let inferred_args_command = ArgsCommand::Patch;
                version = inferred_args_command.bump(&version);
            }
            _ => {
                unreachable!();
            }
        }
        version
    }
}

#[nest_struct]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CargoToml {
    workspace: nest! {
        package: nest! {
            version: String,
        },
    },
}

fn main() {
    let args = Args::parse();
    let cargo_toml = toml::from_str::<CargoToml>(include_str!("../Cargo.toml"))
        .expect("Could not parse Cargo.toml");
    let cwd = cli_run::get_cli_run_cwd();

    let current_version = Version::parse(&cargo_toml.workspace.package.version)
        .expect("Could not parse version from Cargo.toml");
    let commit_message;

    if args.command == ArgsCommand::UpdateReadmeWithCurrentVersion {
        println!("Updating ./README.md ...");
        let readme_content = include_str!("../README.md");

        let pattern = Regex::new(r"System\.Monitor_\d+\.\d+\.\d+").expect("Invalid regex pattern");
        let replacement = format!("System.Monitor_{}", current_version.to_string());

        let new_readme_content = pattern
            .replace_all(readme_content, replacement.as_str())
            .to_string();

        std::fs::write(cwd.join("README.md"), new_readme_content)
            .expect("Could not write new README.md");

        commit_message = format!(
            "Update README.md's download section with version {} [skip ci]",
            current_version
        );
    } else {
        let new_version = args.command.bump(&current_version);
        println!(
            "Bumping version from {} to {}",
            current_version, new_version
        );

        println!("Updating ./Cargo.toml ...");
        let cargo_toml_str = include_str!("../Cargo.toml");
        let new_cargo_toml_str = cargo_toml_str.replace(
            &format!("version = \"{}\"", current_version.to_string()),
            &format!("version = \"{}\"", new_version.to_string()),
        );
        std::fs::write(cwd.join("Cargo.toml"), new_cargo_toml_str)
            .expect("Could not write new Cargo.toml");

        println!("Updating ./Cargo.lock ...");
        cli_run::cli_run("cargo", vec!["generate-lockfile"]);

        println!(
            "Updating ./desktop/web/package.json ...\nUpdating ./desktop/web/package-lock.json ...",
        );
        let cmd = CliRun::new().with_relative_cwd("./desktop/web");
        cmd.run("npm", vec!["version", &new_version.to_string()]);

        println!("Updating ./desktop/tauri.conf.json ...");
        let tauri_conf_json_content = include_str!("../desktop/tauri.conf.json");
        let tauri_pattern = Regex::new(r#""version": "\S+""#).expect("Invalid regex pattern");
        let tauri_replacement = format!("\"version\": \"{}\"", new_version.to_string());
        let new_tauri_conf_json_content = tauri_pattern
            .replace_all(tauri_conf_json_content, tauri_replacement.as_str())
            .to_string();
        std::fs::write(
            cwd.join("desktop/tauri.conf.json"),
            new_tauri_conf_json_content,
        )
        .expect("Could not write new tauri.conf.json");

        commit_message = format!("Bump version to {} [skip ci]", new_version);
    }

    if args.commit {
        println!("Committing changes ...");
        cli_run::cli_run("git", vec!["add", "."]);
        cli_run::cli_run(
            "git",
            vec!["commit", "--allow-empty", "-m", &commit_message],
        );
    } else {
        println!("Skipping commit");
    }
}
