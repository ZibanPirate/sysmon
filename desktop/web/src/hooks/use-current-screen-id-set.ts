import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { CurrentScreenIdSetEvent } from "../../../../common-types/bindings";
import { Loadable } from "../_utils/type";
import { useEventListener } from "./use-event-listener";

// todo-zm: share event names
const EVENT_NAME = "current_screen_id_set_changed";

export function useCurrentScreenIdSet(): {
  currentScreenIdSet: Loadable<string[]>;
  reload: () => void;
} {
  const [currentScreenIdSet, setCurrentScreenIdSet] =
    useState<Loadable<string[]>>(null);

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

  useEventListener<CurrentScreenIdSetEvent>(EVENT_NAME, (event) => {
    setCurrentScreenIdSet(event.payload.updated_current_screen_id_set);
  });

  return { currentScreenIdSet, reload: loadCurrentScreenIdSet };
}
