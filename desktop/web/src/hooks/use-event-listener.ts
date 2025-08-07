import { Event } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { useEffect } from "react";

const hookListeners: Map<
  string,
  Map<string, (event: Event<any>) => void>
> = new Map();

const appWebview = getCurrentWebviewWindow();

export function useEventListener<T>(
  eventName: string,
  listener: (event: Event<T>) => void,
): void {
  useEffect(() => {
    let mapKey = `${eventName}-${Math.random().toString(36).substring(2, 15)}`;
    if (!hookListeners.has(eventName)) {
      hookListeners.set(eventName, new Map());
      appWebview.listen<T>(eventName, (event) => {
        const listeners = hookListeners.get(eventName);
        if (!listeners) {
          // todo-zm: report-error
          console.warn(`No listeners registered for event: ${eventName}`);
          return;
        }
        listeners.forEach((l) => {
          l(event);
        });
      });
    }

    let listeners = hookListeners.get(eventName)!;
    listeners.set(mapKey, listener);
    return () => {
      hookListeners.get(eventName)?.delete(mapKey);
    };
  }, [listener]);
}
