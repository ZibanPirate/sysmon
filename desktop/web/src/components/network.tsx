import { FC } from "react";
import { MonitorEvent } from "../../../../common-types/bindings";

interface NetworkProps {
  networkEvents: Array<MonitorEvent & { type: "Network" }>;
}

export const Network: FC<NetworkProps> = ({ networkEvents }) => {
  return <pre>{JSON.stringify(networkEvents, null, 2)}</pre>;
};
