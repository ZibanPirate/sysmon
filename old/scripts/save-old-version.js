// import { version } from "../package.json";

import { readFileSync, writeFileSync } from "fs";
import { join } from "path";
let version = JSON.parse(readFileSync("package.json")).version;

writeFileSync(join("scripts", ".old-version.txt"), version);
