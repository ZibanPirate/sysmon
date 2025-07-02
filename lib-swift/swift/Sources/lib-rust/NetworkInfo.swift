import AppKit
import Darwin
import Foundation

func getNetworkStatistics() -> (totalSent: UInt64, totalReceived: UInt64) {
    var totalSent: UInt64 = 0
    var totalReceived: UInt64 = 0

    var addrs: UnsafeMutablePointer<ifaddrs>?
    guard getifaddrs(&addrs) == 0 else {
        return (0, 0)
    }
    defer { freeifaddrs(addrs) }

    var cursor = addrs
    while let addr = cursor {
        defer { cursor = addr.pointee.ifa_next }

        let name = String(cString: addr.pointee.ifa_name)

        if name == "lo0" { continue }  // Skip loopback interfaces

        if addr.pointee.ifa_addr.pointee.sa_family == UInt8(AF_LINK) {
            if let data = addr.pointee.ifa_data {
                let ifdata = data.assumingMemoryBound(to: if_data.self)
                totalSent += UInt64(ifdata.pointee.ifi_obytes)
                totalReceived += UInt64(ifdata.pointee.ifi_ibytes)
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
