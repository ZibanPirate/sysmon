import { Event } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { useEffect, useState } from "react";
import { Settings, SettingsEvent } from "../../../../common-types/bindings";
import { invoke } from "@tauri-apps/api/core";

const hookListeners: Map<string, (event: Event<any>) => void> = new Map();
const EVENT_NAME = "settings_changed";

const appWebview = getCurrentWebviewWindow();
appWebview.listen<SettingsEvent>(EVENT_NAME, (event) => {
  hookListeners.forEach((listener) => {
    listener(event);
  });
});

function useSettingsEvent(
  listener: (event: Event<SettingsEvent>) => void,
): void {
  useEffect(() => {
    let mapKey = `${EVENT_NAME}-${Math.random().toString(36).substring(2, 15)}`;
    hookListeners.set(mapKey, listener);
    return () => {
      hookListeners.delete(mapKey);
    };
  }, [listener]);
}

export function useSettings(): {
  settings: Settings | null | "ERROR";
  reload: () => void;
} {
  const [settings, setSettings] = useState<Settings | null | "ERROR">(null);

  async function loadSettings() {
    try {
      setSettings(null);
      const result = await invoke<Settings>("get_settings");
      setSettings(result);
    } catch (error) {
      // todo-zm: capture error in telemetry
      console.error("Failed to load settings:", error);
      setSettings("ERROR");
    }
  }
  useEffect(() => {
    loadSettings();
  }, []);
  useEffect(() => {}, []);
  useSettingsEvent((event) => {
    setSettings(event.payload.updated_settings);
  });

  return { settings, reload: loadSettings };
}
