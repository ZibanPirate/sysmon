import { Event } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { CurrentScreenIdSetEvent } from "../../../../common-types/bindings";

const hookListeners: Map<string, (event: Event<any>) => void> = new Map();
// todo-zm: share event names
const EVENT_NAME = "current_screen_id_set_changed";

const appWebview = getCurrentWebviewWindow();
appWebview.listen<CurrentScreenIdSetEvent>(EVENT_NAME, (event) => {
  hookListeners.forEach((listener) => {
    listener(event);
  });
});

// todo-zm: DRY event listener logic
function useCurrentScreenIdSetEvent(
  listener: (event: Event<CurrentScreenIdSetEvent>) => void,
): void {
  useEffect(() => {
    let mapKey = `${EVENT_NAME}-${Math.random().toString(36).substring(2, 15)}`;
    hookListeners.set(mapKey, listener);
    return () => {
      hookListeners.delete(mapKey);
    };
  }, [listener]);
}

export function useCurrentScreenIdSet(): {
  currentScreenIdSet: string[] | null | "ERROR";
  reload: () => void;
} {
  const [currentScreenIdSet, setCurrentScreenIdSet] = useState<
    string[] | null | "ERROR"
  >(null);

  async function loadCurrentScreenIdSet() {
    try {
      setCurrentScreenIdSet(null);
      const result = await invoke<string[]>("get_current_screen_id_set");
      setCurrentScreenIdSet(result);
    } catch (error) {
      // todo-zm: report-error
      console.error("Failed to load currentScreenIdSet:", error);
      setCurrentScreenIdSet("ERROR");
    }
  }
  useEffect(() => {
    loadCurrentScreenIdSet();
  }, []);
  useEffect(() => {}, []);
  useCurrentScreenIdSetEvent((event) => {
    setCurrentScreenIdSet(event.payload.updated_current_screen_id_set);
  });

  return { currentScreenIdSet, reload: loadCurrentScreenIdSet };
}
