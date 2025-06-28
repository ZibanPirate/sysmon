use cxx_build::CFG;
use glob::glob;

fn main() {
    if !cfg!(target_os = "windows") {
        println!("cargo:warning=Skipping C++ build on non-windows platform.");
        return;
    }

    CFG.include_prefix = "crate-root";

    cxx_build::bridge("src/lib.rs")
        .files(
            glob("cpp/src/*.cc")
                .expect("Failed to read cpp files")
                .filter_map(|entry| match entry {
                    Ok(path) => {
                        let path = path.to_string_lossy().to_string();
                        Some(path)
                    }
                    Err(_) => None,
                }),
        )
        .std("c++17")
        .compile("cxx-librust");

    println!("cargo:rerun-if-changed=cpp/src");
}
