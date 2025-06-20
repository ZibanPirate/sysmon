import AppKit
import Darwin  // For network interface functions
import Foundation
import SwiftRs

public class NetworkInfo: NSObject {
    var totalSent: Int
    var totalReceived: Int

    init(
        totalSent: Int,
        totalReceived: Int
    ) {
        self.totalSent = totalSent
        self.totalReceived = totalReceived
        super.init()
    }
}

func getNetworkStatistics() -> (sent: Int, received: Int) {
    var totalSent: Int = 0
    var totalReceived: Int = 0

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
                totalSent += Int(ifdata.pointee.ifi_obytes)
                totalReceived += Int(ifdata.pointee.ifi_ibytes)
            }
        }
    }

    return (totalSent, totalReceived)
}

@_cdecl("network_info")
public func network_info() -> NSObject {
    let networkStats = getNetworkStatistics()
    let totalSent = networkStats.sent
    let totalReceived = networkStats.received

    let networkInfo = NetworkInfo(
        totalSent: totalSent,
        totalReceived: totalReceived
    )

    return networkInfo
}
