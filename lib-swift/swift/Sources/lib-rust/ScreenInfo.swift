import AppKit
import Foundation

func getScreenInfo() -> RustVec<ScreenInfo> {

    let vec = RustVec<ScreenInfo>()

    let mainScreenHeigh = NSScreen.main?.frame.size.height ?? 0

    for screen in NSScreen.screens {
        let id: String
        if let screenNumber = screen.deviceDescription[NSDeviceDescriptionKey("NSScreenNumber")]
            as? NSNumber
        {
            id = screenNumber.stringValue
        } else {
            id = String(screen.hash)
        }
        let frame = screen.frame
        let safeFrame = screen.visibleFrame
        let isMain = (screen == NSScreen.main)

        // note: in macos, cordinate system starts from the bottom left corner of main screen
        // this only effects the y coordinate, so we need to adjust it
        let y = -(frame.origin.y - (mainScreenHeigh - frame.size.height))

        let screenInfo = ScreenInfo.new(
            id,
            isMain,
            // full
            Rect.new(
                Int64(frame.origin.x),
                Int64(y),
                Int64(frame.size.width), Int64(frame.size.height)
            ),
            // safe
            Rect.new(
                Int64(safeFrame.origin.x),
                // workaround-zm: safeFrame.origin.y does not reflect the actual position of the safe-area, but instead is always 0
                // to work around this, we use the frame's origin.y plus the difference between the safeFrame's height and the frame's height
                Int64(y + frame.size.height - safeFrame.size.height),
                Int64(safeFrame.size.width), Int64(safeFrame.size.height)
            )

        )

        vec.push(value: screenInfo)
    }

    return vec
}
