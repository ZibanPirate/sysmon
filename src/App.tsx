import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { EventCallback, listen } from "@tauri-apps/api/event";
import { useEffect, useMemo, useState } from "react";
import { error } from "@tauri-apps/plugin-log";
import { Chart } from "react-charts";
import debounce from "lodash/debounce";

type NetworkSpeed = {
  receivedBytes: number;
  sentBytes: number;
  utcTimestamp: number;
};

let callback: EventCallback<unknown> = (_event) => {
  // info(JSON.stringify(_event, null, 2));
};

listen("network-info", (...args) => callback(...args)).catch((err) => {
  error(String(err));
});

type Series = {
  label: string;
  data: Array<Pick<NetworkSpeed, "utcTimestamp"> & { speed: number }>;
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

let observer = new ResizeObserver(resizeTriggerDebounced);
observer.observe(document.body);

let lastSpeedsFromLocalStorage: NetworkSpeed[];
try {
  lastSpeedsFromLocalStorage = JSON.parse(
    localStorage.getItem("lastSpeeds") || "[]"
  );

  let now = new Date().getTime();
  lastSpeedsFromLocalStorage = lastSpeedsFromLocalStorage.filter(
    (speed) => now - speed.utcTimestamp < 60 * 1000
  );
} catch (err) {
  lastSpeedsFromLocalStorage = [];
  error(String(err));
}

function App() {
  const [speed, setSpeed] = useState<NetworkSpeed>({
    receivedBytes: 0,
    sentBytes: 0,
    utcTimestamp: new Date().getTime(),
  });
  const [last_speeds, setLastSpeeds] = useState<NetworkSpeed[]>(
    lastSpeedsFromLocalStorage
  );
  useEffect(() => {
    let newLastSpeeds = [...last_speeds, speed].slice(-60);
    localStorage.setItem("lastSpeeds", JSON.stringify(newLastSpeeds));
    setLastSpeeds(newLastSpeeds);
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
    () =>
      [
        {
          label: "Download Speed",
          data: last_speeds.map(({ utcTimestamp, receivedBytes }) => ({
            utcTimestamp,
            speed: -receivedBytes,
          })),
        },
        {
          label: "Upload Speed",
          data: last_speeds.map(({ utcTimestamp, sentBytes }) => ({
            utcTimestamp,
            speed: -sentBytes,
          })),
        },
      ] satisfies Series[],
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
            min: data[0]?.data[0]?.utcTimestamp
              ? new Date(data[0]?.data[0]?.utcTimestamp)
              : undefined,
            show: false,
          },
          defaultColors: ["#09f9", "#f099"],
          secondaryAxes: [
            {
              getValue: (datum) => datum.speed,
              scaleType: "linear",
              showGrid: false,
              formatters: { scale: () => "" },
              show: false,
              stacked: true,
            },
          ],
          primaryCursor: { show: false },
          secondaryCursor: { show: false },
          tooltip: { show: false },
          padding: 0,
        }}
      />
    </div>
  );
}

export default App;
