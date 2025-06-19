use swift_rs::SwiftLinker;

fn main() {
    SwiftLinker::new("10.13")
        .with_package("macos", "./swift-package")
        .link();
}
