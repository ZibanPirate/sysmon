import AppKit
import Foundation
import SwiftRs

public class ScreenInfo: NSObject {
    var x: Int
    var y: Int
    var width: Int
    var height: Int
    var isMain: Bool
    var safe_x: Int
    var safe_y: Int
    var safe_width: Int
    var safe_height: Int

    init(
        x: Int, y: Int, width: Int, height: Int, isMain: Bool, safe_x: Int = 0, safe_y: Int = 0,
        safe_width: Int = 0, safe_height: Int = 0
    ) {
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.isMain = isMain
        self.safe_x = safe_x
        self.safe_y = safe_y
        self.safe_width = safe_width
        self.safe_height = safe_height
        super.init()
    }
}

@_cdecl("desktop_info")
public func desktop_info() -> SRObjectArray {

    var screenInfos: [ScreenInfo] = []

    for screen in NSScreen.screens {
        let frame = screen.frame
        let safeFrame = screen.visibleFrame
        let isMain = (screen == NSScreen.main)

        let screenInfo = ScreenInfo(
            x: Int(frame.origin.x),
            y: Int(frame.origin.y),
            width: Int(frame.size.width),
            height: Int(frame.size.height),
            isMain: isMain,
            safe_x: Int(safeFrame.origin.x),
            safe_y: Int(safeFrame.origin.y),
            safe_width: Int(safeFrame.size.width),
            safe_height: Int(safeFrame.size.height)
        )
        screenInfos.append(screenInfo)
    }

    return SRObjectArray(screenInfos)
}
