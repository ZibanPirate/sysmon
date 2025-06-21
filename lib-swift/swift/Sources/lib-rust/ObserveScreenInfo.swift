import AppKit
import Cocoa
import Foundation

public class ScreenChangeObserver {
    public typealias ChangeHandler = () -> Void

    private var handler: ChangeHandler?
    private var isObserving = false

    public init(handler: @escaping ChangeHandler) {
        self.handler = handler
    }

    deinit {
        stopObserving()
    }

    public func startObserving() {
        guard !isObserving else { return }

        NotificationCenter.default.addObserver(
            self,
            selector: #selector(screenParametersChanged(_:)),
            name: NSApplication.didChangeScreenParametersNotification,
            object: nil
        )
        isObserving = true
    }

    public func stopObserving() {
        guard isObserving else { return }

        NotificationCenter.default.removeObserver(
            self,
            name: NSApplication.didChangeScreenParametersNotification,
            object: nil
        )
        isObserving = false
    }

    @objc private func screenParametersChanged(_ notification: Notification) {
        handler?()
    }

    public func updateHandler(_ newHandler: @escaping ChangeHandler) {
        handler = newHandler
    }
}

nonisolated(unsafe) let observer = ScreenChangeObserver {
    messageFromSwift(SwiftMessage.new_screen_info_changed())
}

func startObservingScreenInfo() {
    observer.startObserving()
}
