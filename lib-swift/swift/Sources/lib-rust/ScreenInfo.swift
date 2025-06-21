import AppKit
import Foundation

func getScreenInfo() -> RustVec<ScreenInfo> {

    let vec = RustVec<ScreenInfo>()

    for screen in NSScreen.screens {
        let frame = screen.frame
        let safeFrame = screen.visibleFrame
        let isMain = (screen == NSScreen.main)

        let screenInfo = ScreenInfo.new(
            isMain,
            // full
            Rect.new(
                Int64(frame.origin.x), Int64(frame.origin.y),
                Int64(frame.size.width), Int64(frame.size.height)
            ),
            // safe
            Rect.new(
                Int64(safeFrame.origin.x), Int64(safeFrame.origin.y),
                Int64(safeFrame.size.width), Int64(safeFrame.size.height)
            )

        )

        vec.push(value: screenInfo)
    }

    return vec
}
