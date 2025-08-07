import { Settings } from "../../../../common-types/bindings";
import { Loadable } from "./type";

export function extractPositionForScreenIdSet(
  settings: Loadable<Settings>,
  currentScreenIdSet: Loadable<string[]>,
) {
  if (
    settings === "ERROR" ||
    settings === null ||
    currentScreenIdSet === "ERROR" ||
    currentScreenIdSet === null
  )
    return;

  const currentScreenIdSetString = currentScreenIdSet.join("-");

  return settings.network_widget.position_per_screen_set.find((pos) => {
    return pos.screen_id_set.join("-") === currentScreenIdSetString;
  });
}
