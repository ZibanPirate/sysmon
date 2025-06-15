import { useState } from "react";
import "./App.css";
import { MonitorEvent, useMonitorEvent } from "./hooks/use-monitor-event";

export function App() {
  const [events, setEvents] = useState<{
    count: number;
    lastEvent: MonitorEvent | null;
  }>({ count: 0, lastEvent: null });

  useMonitorEvent((event) => {
    setEvents((prevEvents) => ({
      count: prevEvents.count + 1,
      lastEvent: event.payload,
    }));
  });

  return (
    <main className="container">
      <pre>{JSON.stringify(events, null, 2)}</pre>
    </main>
  );
}
