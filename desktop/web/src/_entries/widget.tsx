import React, { useMemo } from "react";
import ReactDOM from "react-dom/client";
import { useState } from "react";
import { useMonitorEvent } from "../hooks/use-monitor-event";
import { MonitorEvent } from "../../../../common-types/bindings";
import { Network } from "../components/network";
import { useSettings } from "../hooks/use-settings";
import "../_utils/import-daisyui.css";
import "../_utils/transparent-window.css";
import { useCurrentScreenIdSet } from "../hooks/use-current-screen-id-set";
import { extractPositionForScreenIdSet } from "../_utils/extract-position-for-screen-id-set";

const MAX_EVENTS = 50;

function App() {
  const [events, setEvents] = useState<MonitorEvent[]>(() =>
    Array.from({ length: MAX_EVENTS }, () => ({
      type: "Network",
      content: { sent: 0, received: 0 },
    })),
  );

  useMonitorEvent((event) => {
    setEvents((prevEvents) => {
      let newEvents = [...prevEvents, event.payload];

      if (newEvents.length > MAX_EVENTS) {
        newEvents = newEvents.slice(newEvents.length - MAX_EVENTS);
      }

      return newEvents;
    });
  });

  const { settings, reload } = useSettings();
  const { currentScreenIdSet } = useCurrentScreenIdSet();
  const position = useMemo(
    () => extractPositionForScreenIdSet(settings, currentScreenIdSet)?.position,
    [currentScreenIdSet, settings],
  );

  if (settings === null || !position) {
    return (
      <div className="flex flex-col w-full justify-center p-6">
        <progress className="progress w-full"></progress>
      </div>
    );
  }

  if (settings === "ERROR") {
    return (
      <div className="flex flex-col w-full justify-center items-center">
        <button className="btn btn-sm" onClick={reload}>
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col w-full h-full bg-transparent">
      <Network
        networkEvents={events.filter((event) => event.type === "Network")}
        position={position}
      />
    </div>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
