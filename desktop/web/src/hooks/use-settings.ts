import { useEffect, useState } from "react";
import { Settings, SettingsEvent } from "../../../../common-types/bindings";
import { invoke } from "@tauri-apps/api/core";
import { Loadable } from "../_utils/type";
import { useEventListener } from "./use-event-listener";

const EVENT_NAME = "settings_changed";

export function useSettings(): {
  settings: Loadable<Settings>;
  reload: () => void;
} {
  const [settings, setSettings] = useState<Loadable<Settings>>(null);

  async function loadSettings() {
    try {
      setSettings(null);
      const result = await invoke<Settings>("get_settings");
      setSettings(result);
    } catch (error) {
      // todo-zm: report-error
      console.error("Failed to load settings:", error);
      setSettings("ERROR");
    }
  }

  useEffect(() => {
    loadSettings();
  }, []);

  useEventListener<SettingsEvent>(EVENT_NAME, (event) => {
    setSettings(event.payload.updated_settings);
  });

  return { settings, reload: loadSettings };
}
