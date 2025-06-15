import { useState } from "react";
import { useMonitorEvent } from "../hooks/use-monitor-event";
import { MonitorEvent } from "../../../../common-types/bindings";
import { Network } from "../components/network";
import "./style.css";

const MAX_EVENTS = 100;

export function App() {
  const [events, setEvents] = useState<MonitorEvent[]>([]);

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
