// swift-tools-version: 6.1
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "lib-rust",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "rust",
            type: .static,
            targets: ["lib-rust"])
    ],
    targets: [
        .target(
            name: "lib-rust",
            swiftSettings: [
                .define("SWIFT_PACKAGE_ALWAYS_EMBED_SWIFT_STANDARD_LIBRARIES")
            ])
    ],
)
