use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "crate-root";

    cxx_build::bridge("src/lib.rs")
        .file("cpp/src/lib.cc")
        .std("c++17")
        .compile("cxx-librust");

    println!("cargo:rerun-if-changed=cpp/src");
}
