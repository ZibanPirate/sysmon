import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { EventCallback, listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { info, error } from "@tauri-apps/plugin-log";

type NetworkSpeed = {
  speed: number;
  utcTimestamp: number;
};

info(JSON.stringify("event", null, 2));

let callback: EventCallback<unknown> = (_event) => {
  // info(JSON.stringify(_event, null, 2));
};

try {
  await listen("network-info", (...args) => callback(...args));
} catch (err) {
  error(String(err));
}

function App() {
  const [speed, setSpeed] = useState<NetworkSpeed>({
    speed: 0,
    utcTimestamp: 0,
  });

  useEffect(() => {
    callback = (event) => {
      // info(`zako: ${JSON.stringify(event, null, 2)}`);
      setSpeed(event.payload as NetworkSpeed);
    };

    return () => {
      callback = () => {};
    };
  }, []);

  return (
    <div className="container">
      <h2>Network Speed</h2>
      <pre>{JSON.stringify(speed, null, 2)}</pre>

      <button onClick={() => invoke("close")}>Close</button>
    </div>
  );
}

export default App;
