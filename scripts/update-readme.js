import { readFileSync, writeFileSync } from "fs";
import { join } from "path";
let oldVersion = String(readFileSync(join("scripts", ".old-version.txt")));
let newVersion = JSON.parse(readFileSync("package.json")).version;

let readme = readFileSync("README.md", "utf-8");
let regExp = new RegExp(oldVersion.replace(/\./g, "\\."), "g");
readme = readme.replace(regExp, newVersion);
writeFileSync("README.md", readme);
