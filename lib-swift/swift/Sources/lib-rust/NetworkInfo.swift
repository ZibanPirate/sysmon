import AppKit
import Darwin  // For network interface functions
import Foundation

func getNetworkStatistics() -> (totalSent: UInt32, totalReceived: UInt32) {
    var totalSent: UInt32 = 0
    var totalReceived: UInt32 = 0

    var addrs: UnsafeMutablePointer<ifaddrs>?
    guard getifaddrs(&addrs) == 0 else {
        return (0, 0)
    }
    defer { freeifaddrs(addrs) }

    var cursor = addrs
    while let addr = cursor {
        defer { cursor = addr.pointee.ifa_next }

        let name = String(cString: addr.pointee.ifa_name)
        // Skip loopback interfaces
        if name == "lo0" { continue }

        if addr.pointee.ifa_addr.pointee.sa_family == UInt8(AF_LINK) {
            // For AF_LINK interfaces, the ifa_data pointer points to if_data
            if let data = addr.pointee.ifa_data {
                let ifdata = data.assumingMemoryBound(to: if_data.self)
                totalSent += ifdata.pointee.ifi_obytes
                totalReceived += ifdata.pointee.ifi_ibytes
            }
        }
    }

    return (totalSent, totalReceived)
}

func getNetworkInfo() -> RustVec<NetworkInfo> {
    let vec = RustVec<NetworkInfo>()

    let (totalSent, totalReceived) = getNetworkStatistics()

    vec.push(value: NetworkInfo.new(totalSent, totalReceived))

    return vec
}
