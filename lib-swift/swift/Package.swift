// swift-tools-version: 6.1
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "lib-rust",
    products: [
        .library(
            name: "rust",
            type: .static,
            targets: ["lib-rust"])
    ],
    targets: [
        .target(
            name: "lib-rust")
    ]
)
