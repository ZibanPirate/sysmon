import { Event } from "@tauri-apps/api/event";
import { MonitorEvent } from "../../../../common-types/bindings";
import { useEventListener } from "./use-event-listener";

const EVENT_NAME = "monitor_event";

export function useMonitorEvent(
  listener: (event: Event<MonitorEvent>) => void,
): void {
  useEventListener(EVENT_NAME, listener);
}
