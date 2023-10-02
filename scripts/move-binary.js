// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * This script is used to rename the binary with the platform specific postfix.
 * When `tauri build` is ran, it looks for the binary name appended with the platform specific postfix.
 */

import { execa } from "execa";
import fs from "fs";

const nodeServerBinName = "WarframeServer";
const launchBinName = "Warframe.x64";

let extension = "";
if (process.platform === "win32") {
    extension = ".exe";
}

async function createLaunchBin(targetTriple) {
    fs.writeFile(`src-tauri/binaries/${launchBinName}-${targetTriple}${extension}`, "", (err) => {
        if (err) throw err;
    });
}

async function main() {
    const rustInfo = (await execa("rustc", ["-vV"])).stdout;
    const targetTriple = /host: (\S+)/g.exec(rustInfo)[1];

    if (!targetTriple) {
        console.error("Failed to determine platform target triple");
    }

    await createLaunchBin(targetTriple);

    fs.renameSync(
        `src-tauri/binaries/${nodeServerBinName}${extension}`,
        `src-tauri/binaries/${nodeServerBinName}-${targetTriple}${extension}`,
    );
}

main().catch((e) => {
    throw e;
});
