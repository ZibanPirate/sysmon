import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { EventCallback, listen } from "@tauri-apps/api/event";
import { useEffect, useMemo, useState } from "react";
import { error } from "@tauri-apps/plugin-log";
import { Chart } from "react-charts";
import debounce from "lodash/debounce";

type NetworkSpeed = {
  packets: number;
  utcTimestamp: number;
};

let callback: EventCallback<unknown> = (_event) => {
  // info(JSON.stringify(_event, null, 2));
};

try {
  await listen("network-info", (...args) => callback(...args));
} catch (err) {
  error(String(err));
}

type Series = {
  label: string;
  data: NetworkSpeed[];
};

let resizeTriggerDebounced = debounce(() => {
  invoke("resize", {
    width: document.body.clientWidth,
    height: document.body.clientHeight,
  });
  setTimeout(() => {
    invoke("resize", {
      width: document.body.clientWidth,
      height: document.body.clientHeight,
    });
  }, 1000);
}, 2000);

// observe body and trigger resize event
let observer = new ResizeObserver(resizeTriggerDebounced);
observer.observe(document.body);

function App() {
  const [speed, setSpeed] = useState<NetworkSpeed>({
    packets: 0,
    utcTimestamp: new Date().getTime(),
  });
  const [last_speeds, setLastSpeeds] = useState<NetworkSpeed[]>([]);
  useEffect(() => {
    setLastSpeeds([...last_speeds, speed].slice(-20));
  }, [speed]);

  useEffect(() => {
    callback = (event) => {
      // info(`zako: ${JSON.stringify(event, null, 2)}`);
      setSpeed(event.payload as NetworkSpeed);
    };

    return () => {
      callback = () => {};
    };
  }, []);

  const data = useMemo(
    () => [{ label: "Network Speed", data: last_speeds }] satisfies Series[],
    [last_speeds]
  );

  return (
    <div className="container">
      <Chart
        className="chart"
        options={{
          data,
          primaryAxis: {
            getValue: (datum) => new Date(datum.utcTimestamp),
            scaleType: "time",
            showGrid: false,
            formatters: {
              scale: () => "",
            },
            show: false,
          },
          secondaryAxes: [
            {
              getValue: (datum) => datum.packets,
              scaleType: "linear",
              showGrid: false,
              formatters: {
                scale: () => "",
              },
              show: false,
            },
          ],
          dark: true,
          primaryCursor: { show: false },
          secondaryCursor: { show: false },
          tooltip: { show: false },
        }}
      />
    </div>
  );
}

export default App;
