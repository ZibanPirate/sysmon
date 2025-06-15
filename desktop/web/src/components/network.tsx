import { FC, useMemo } from "react";
import { MonitorEvent } from "../../../../common-types/bindings";

interface NetworkProps {
  networkEvents: Array<MonitorEvent & { type: "Network" }>;
}

const MIN_VALUE_BYTES = 1_000_000;
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
    let sentPolylinePoints = "";
    networkEvents.forEach((event, index) => {
      const x = (index * WIDTH) / networkEvents.length;
      const yReceived = (event.content.received * HEIGHT) / max;
      const ySent =
        ((event.content.sent + event.content.received) * HEIGHT) / max;
      receivedPolylinePoints += `${x.toFixed(0)},${yReceived.toFixed(0)} `;
      sentPolylinePoints += `${x.toFixed(0)},${ySent.toFixed(0)} `;
    });
    return { receivedPolylinePoints, sentPolylinePoints };
  }, [networkEvents]);
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox={`0 0 ${WIDTH} ${HEIGHT}`}
      style={{
        background: "red",
        width: "100%",
      }}
    >
      <polyline points={sentPolylinePoints} fill="none" stroke="blue" />
      <polyline points={receivedPolylinePoints} fill="none" stroke="black" />
    </svg>
  );
};
