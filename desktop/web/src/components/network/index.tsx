import { FC, useMemo } from "react";
import { MonitorEvent } from "../../../../../common-types/bindings";
import styles from "./style.module.css";

interface NetworkProps {
  networkEvents: Array<MonitorEvent & { type: "Network" }>;
}

const MIN_VALUE_BYTES = 100_000;
const ASPECT_RATIO = 6 / 2;
const WIDTH = 100;
const HEIGHT = WIDTH / ASPECT_RATIO;

export const Network: FC<NetworkProps> = ({ networkEvents }) => {
  const { receivedPolylinePoints, sentPolylinePoints } = useMemo(() => {
    const max = Math.max(
      ...networkEvents.map(
        (event) => event.content.received + event.content.sent
      ),
      MIN_VALUE_BYTES
    );
    let receivedPolylinePoints = "";
    let receivedPolylinePointsReversed = "";
    let sentPolylinePoints = "";
    networkEvents.forEach((event, index) => {
      const x = Math.round((index * WIDTH) / (networkEvents.length - 1));
      const yReceived = Math.round((event.content.received * HEIGHT) / max);
      const ySent = Math.round(
        ((event.content.sent + event.content.received) * HEIGHT) / max
      );

      receivedPolylinePoints += `${x},${yReceived} `;
      receivedPolylinePointsReversed = ` ${x},${yReceived}${receivedPolylinePointsReversed}`;
      sentPolylinePoints += `${x},${ySent} `;
    });

    // close the polylines
    receivedPolylinePoints = `0,0 ${receivedPolylinePoints} ${WIDTH},0`;
    // stack the sent polyline on top of the received one and also close it
    sentPolylinePoints = `0,0 ${sentPolylinePoints} ${receivedPolylinePointsReversed} 0,0`;

    return { receivedPolylinePoints, sentPolylinePoints };
  }, [networkEvents]);
  return (
    <>
      <pre>
        {JSON.stringify(
          {
            firstEvent: receivedPolylinePoints.slice(0, 10),
            lastEvent: receivedPolylinePoints.slice(-10),
          },
          null,
          2
        )}
      </pre>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox={`0 0 ${WIDTH} ${HEIGHT}`}
      >
        <polyline points={sentPolylinePoints} className={styles.polylineSent} />
        <polyline
          points={receivedPolylinePoints}
          className={styles.polylineReceived}
        />
      </svg>
    </>
  );
};
