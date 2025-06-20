import AppKit
import Foundation
import SwiftRs

public class ScreenInfo: NSObject {
    var x: Int
    var y: Int
    var width: Int
    var height: Int
    var isMain: Bool
    var safeX: Int
    var safeY: Int
    var safeWidth: Int
    var safeHeight: Int

    init(
        x: Int, y: Int, width: Int, height: Int, isMain: Bool, safeX: Int, safeY: Int,
        safeWidth: Int, safeHeight: Int
    ) {
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.isMain = isMain
        self.safeX = safeX
        self.safeY = safeY
        self.safeWidth = safeWidth
        self.safeHeight = safeHeight
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
            safeX: Int(safeFrame.origin.x),
            safeY: Int(safeFrame.origin.y),
            safeWidth: Int(safeFrame.size.width),
            safeHeight: Int(safeFrame.size.height)
        )
        screenInfos.append(screenInfo)
    }

    return SRObjectArray(screenInfos)
}
