use std::{path::PathBuf, process::Command};

const SWIFT_STATIC_LIB_NAME: &str = "rust"; // this looks for `librust.a`

fn main() {
    let (rust_crate_dir, swift_build_dir) = swift_dirs();

    let mut cmd = Command::new("swift-bridge-cli");
    cmd.current_dir(&rust_crate_dir)
        .arg("parse-bridges")
        .arg("--crate-name")
        .arg("lib-swift")
        .arg("-f")
        .arg("src/lib.rs")
        .arg("-o")
        .arg("swift/Sources/lib-rust/generated");

    let exit_status = cmd.spawn().unwrap().wait_with_output().unwrap();
    if !exit_status.status.success() {
        panic!(
            r#"\nStderr: {}\nStdout: {}\n"#,
            String::from_utf8(exit_status.stderr).unwrap(),
            String::from_utf8(exit_status.stdout).unwrap(),
        )
    }

    let mut cmd = Command::new("swift");
    cmd.current_dir(&rust_crate_dir.join("swift"))
        .arg("build")
        .arg("-Xswiftc")
        .arg("-static")
        .arg("-Xswiftc")
        .arg("-import-objc-header")
        .arg("-Xswiftc")
        .arg("Sources/lib-rust/bridging-header.h");

    if is_release_build() {
        cmd.args(&["-c", "release"]);
    }

    let exit_status = cmd.spawn().unwrap().wait_with_output().unwrap();
    if !exit_status.status.success() {
        panic!(
            r#"\nStderr: {}\nStdout: {}\n"#,
            String::from_utf8(exit_status.stderr).unwrap(),
            String::from_utf8(exit_status.stdout).unwrap(),
        )
    }

    println!("cargo:rustc-link-lib=static={}", SWIFT_STATIC_LIB_NAME);
    println!(
        "cargo:rustc-link-search={}",
        swift_build_dir.to_str().unwrap()
    );
    xcode_link_dir().iter().for_each(|dir| {
        println!("cargo:rustc-link-search={}", dir.to_str().unwrap());
    });
}

fn is_release_build() -> bool {
    std::env::var("PROFILE").unwrap() == "release"
}

fn swift_dirs() -> (PathBuf, PathBuf) {
    let rust_crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let debug_or_release = if is_release_build() {
        "release"
    } else {
        "debug"
    };

    let build_dir =
        PathBuf::from(&rust_crate_dir).join(format!("swift/.build/{}", debug_or_release));

    (PathBuf::from(&rust_crate_dir), build_dir)
}

fn xcode_link_dir() -> Vec<PathBuf> {
    let xcode_path = if let Ok(output) = std::process::Command::new("xcode-select")
        .arg("--print-path")
        .output()
    {
        String::from_utf8(output.stdout.as_slice().into())
            .unwrap()
            .trim()
            .to_string()
    } else {
        "/Applications/Xcode.app/Contents/Developer".to_string()
    };

    vec![
        format!(
            "{}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx/",
            &xcode_path
        )
        .into(),
        "/usr/lib/swift".into(),
    ]
}
