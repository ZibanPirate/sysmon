import React, { Fragment, useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import "../_utils/import-daisyui.css";
import {
  Settings,
  SettingsNetworkWidgetPosition,
  SettingsTrayContent,
} from "../../../../common-types/bindings";
import { invoke } from "@tauri-apps/api/core";

function App() {
  let [settings, setSettings] = useState<Settings | null | "ERROR">(null);
  let saveSettings = async (updatedSettings: Settings) => {
    try {
      let saved_settings = await invoke<Settings>("set_settings", {
        updatedSettings,
      });
      setSettings(saved_settings);
    } catch (error) {
      // todo-zm: capture error in telemetry
      console.error("Failed to save settings:", error);
    }
  };

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

  if (settings === null) {
    return (
      <div className="flex flex-col w-full justify-center p-6">
        <progress className="progress w-full"></progress>
      </div>
    );
  }

  if (settings === "ERROR") {
    return (
      <div className="flex flex-col w-full justify-center items-center p-6">
        <div className="text-red-500">Failed to load settings</div>
        <button className="btn mt-4" onClick={loadSettings}>
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="flex-1 tabs tabs-border">
      <input
        type="radio"
        name="tab"
        className="tab"
        aria-label="General"
        defaultChecked
      />
      <div className="tab-content bg-base-100 border-base-300 p-6">
        <div className="flex flex-col gap-4">
          <label className="label gap-4">
            <input
              type="checkbox"
              className="toggle"
              checked={settings.general.start_on_boot}
              onChange={(e) => {
                saveSettings({
                  ...settings,
                  general: {
                    ...settings.general,
                    start_on_boot: e.target.checked,
                  },
                });
              }}
            />
            Start on boot
          </label>
          <label className="label gap-4">
            <input
              type="checkbox"
              className="toggle"
              checked={settings.general.send_usage_telemetry}
              onChange={(e) => {
                saveSettings({
                  ...settings,
                  general: {
                    ...settings.general,
                    send_usage_telemetry: e.target.checked,
                  },
                });
              }}
            />
            Send usage telemetry
          </label>
        </div>
      </div>

      <input
        type="radio"
        name="tab"
        className="tab"
        aria-label="Network Widget"
      />
      <div className="tab-content bg-base-100 border-base-300 p-6">
        <div className="flex flex-col gap-4">
          <label className="label gap-4">
            <input
              type="checkbox"
              className="toggle"
              checked={settings.network_widget.enabled}
              onChange={(e) => {
                saveSettings({
                  ...settings,
                  network_widget: {
                    ...settings.network_widget,
                    enabled: e.target.checked,
                  },
                });
              }}
            />
            Enable network widget
          </label>
          <div className="label gap-4">
            Position
            <div>
              {Object.values(SettingsNetworkWidgetPosition).map(
                (position, index) => (
                  <Fragment key={position}>
                    <button
                      className={`btn btn-sm ${
                        position === settings.network_widget.position
                          ? "btn-soft"
                          : "btn-ghost"
                      }`}
                      onClick={() => {
                        saveSettings({
                          ...settings,
                          network_widget: {
                            ...settings.network_widget,
                            position: position,
                          },
                        });
                      }}
                    >
                      <svg
                        style={{
                          transform: `scaleX(${
                            position.includes("Right") ? -1 : 1
                          }) scaleY(${position.includes("Top") ? 1 : -1})`,
                        }}
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        strokeWidth={1.5}
                        stroke="currentColor"
                        className="size-2"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          d="m19.5 19.5-15-15m0 0v11.25m0-11.25h11.25"
                        />
                      </svg>
                    </button>
                    {index === 1 && <br className="flex flex-auto" />}
                  </Fragment>
                )
              )}
            </div>
          </div>
          <label className="label gap-4">
            <input
              type="checkbox"
              className="toggle"
              checked={settings.network_widget.safe_area}
              onChange={(e) => {
                saveSettings({
                  ...settings,
                  network_widget: {
                    ...settings.network_widget,
                    safe_area: e.target.checked,
                  },
                });
              }}
            />
            Within safe area
          </label>

          <label className="label gap-4">
            Size
            <input
              type="range"
              min="100"
              max="400"
              value={settings.network_widget.size}
              className="range"
              onChange={(e) => {
                saveSettings({
                  ...settings,
                  network_widget: {
                    ...settings.network_widget,
                    size: Number(e.target.value),
                  },
                });
              }}
            />
          </label>
        </div>
      </div>

      <input type="radio" name="tab" className="tab" aria-label="Tray" />
      <div className="tab-content bg-base-100 border-base-300 p-6">
        <div className="label gap-4">
          Shown on title
          <div className="join">
            {Object.values(SettingsTrayContent).map((content) => (
              <input
                className={`join-item btn btn-ghost ${
                  settings.tray.content === content ? "btn-active" : ""
                }`}
                type="radio"
                name="options"
                aria-label={content}
                key={content}
                checked={settings.tray.content === content}
                onChange={() => {
                  saveSettings({
                    ...settings,
                    tray: {
                      ...settings.tray,
                      content: content,
                    },
                  });
                }}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
