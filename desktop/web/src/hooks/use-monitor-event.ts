import { Event } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { useEffect } from "react";
import { MonitorEvent } from "../../../../common-types/bindings";

const hookListeners: Map<string, (event: Event<any>) => void> = new Map();
const EVENT_NAME = "monitor_event";

const appWebview = getCurrentWebviewWindow();
appWebview.listen<MonitorEvent>(EVENT_NAME, (event) => {
  hookListeners.forEach((listener) => {
    listener(event);
  });
});

export function useMonitorEvent(
  listener: (event: Event<MonitorEvent>) => void,
): void {
  useEffect(() => {
    let mapKey = `${EVENT_NAME}-${Math.random().toString(36).substring(2, 15)}`;
    hookListeners.set(mapKey, listener);
    return () => {
      hookListeners.delete(mapKey);
    };
  }, [listener]);
}
