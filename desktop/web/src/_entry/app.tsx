import { useState } from "react";
import { useMonitorEvent } from "../hooks/use-monitor-event";
import { MonitorEvent } from "../../../../common-types/bindings";
import { Network } from "../components/network";
import "./style.css";

const MAX_EVENTS = 50;

export function App() {
  const [events, setEvents] = useState<MonitorEvent[]>(() =>
    Array.from({ length: MAX_EVENTS }, () => ({
      type: "Network",
      content: { sent: 0, received: 0 },
    }))
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

  return (
    <Network
      networkEvents={events.filter((event) => event.type === "Network")}
    />
  );
}
