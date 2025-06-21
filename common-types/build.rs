use std::process::Command;

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut cmd = Command::new("typeshare");
    cmd.current_dir(&crate_dir)
        .arg("./")
        .arg("--lang=typescript")
        .arg("--output-file=./bindings/index.ts");

    let exit_status = cmd.spawn().unwrap().wait_with_output().unwrap();
    if !exit_status.status.success() {
        panic!(
            r#"\nStderr: {}\nStdout: {}\n"#,
            String::from_utf8(exit_status.stderr).unwrap(),
            String::from_utf8(exit_status.stdout).unwrap(),
        )
    }
}
