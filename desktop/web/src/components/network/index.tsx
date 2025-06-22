import { FC, useMemo } from "react";
import { MonitorEvent, Settings } from "../../../../../common-types/bindings";
import styles from "./style.module.css";

interface NetworkProps {
  networkEvents: Array<MonitorEvent & { type: "Network" }>;
  position: Settings["network_widget"]["position"];
}

const MIN_VALUE_BYTES = 10_000;

export const Network: FC<NetworkProps> = ({ networkEvents, position }) => {
  const WIDTH = window.innerWidth;
  const HEIGHT = window.innerHeight;

  const { receivedPath, sentPath } = useMemo(() => {
    let max = Math.max(
      ...networkEvents.map(
        (event) => event.content.received + event.content.sent
      ),
      MIN_VALUE_BYTES
    );
    max = max * 2;

    // Helper function to create cubic bezier commands
    const createBezierPath = (
      points: Array<{ x: number; y: number }>
    ): string => {
      if (points.length < 2) return "";

      let path = `M ${points[0].x},${points[0].y}`;

      for (let i = 0; i < points.length - 1; i++) {
        const current = points[i];
        const next = points[i + 1];

        // Control points for cubic bezier
        const cp1x = current.x + (next.x - current.x) / 3;
        const cp1y = current.y;
        const cp2x = next.x - (next.x - current.x) / 3;
        const cp2y = next.y;

        path += ` C ${cp1x},${cp1y} ${cp2x},${cp2y} ${next.x},${next.y}`;
      }

      return path;
    };

    // Generate points for received data
    const receivedPoints = [{ x: 0, y: 0 }];
    const sentPoints = [{ x: 0, y: 0 }];

    networkEvents.forEach((event, index) => {
      const x = Math.round((index * WIDTH) / (networkEvents.length - 1));
      const yReceived = Math.round((event.content.received * HEIGHT) / max);
      const ySent = Math.round(
        ((event.content.sent + event.content.received) * HEIGHT) / max
      );

      receivedPoints.push({ x, y: yReceived });
      sentPoints.push({ x, y: ySent });
    });

    // Add final points to close the paths
    receivedPoints.push({ x: WIDTH, y: 0 });

    // Create reversed received points for sent path closure
    const receivedPointsReversed = [...receivedPoints]
      .slice(1, -1) // Remove first and last point (0,0 and WIDTH,0)
      .reverse()
      .map((point) => ({ x: point.x, y: point.y }));

    // Close the sent path
    sentPoints.push(...receivedPointsReversed, { x: 0, y: 0 });

    // Create bezier paths
    const receivedPath = createBezierPath(receivedPoints);
    const sentPath = createBezierPath(sentPoints);

    return { receivedPath, sentPath };
  }, [networkEvents]);

  return (
    <svg
      className={styles.fadeToLeft}
      xmlns="http://www.w3.org/2000/svg"
      viewBox={`0 0 ${WIDTH} ${HEIGHT}`}
      style={{
        transform: `scaleX(${position.includes("Left") ? -1 : 1}) scaleY(${
          position.includes("Top") ? 1 : -1
        })`,
      }}
    >
      <path d={sentPath} className={styles.polylineSent} />
      <path d={receivedPath} className={styles.polylineReceived} />
    </svg>
  );
};
