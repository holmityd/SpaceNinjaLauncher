import { access, readFile, writeFile } from "fs/promises";

(async () => {
    const filePath = "build/index.js";

    const comment = `/*! mongodb workaround */`;

    try {
        await access("build/mongod.exe");
    } catch (error) {
        return console.error("error: mongod.exe not in build folder");
    }

    let contentToPrepend = await readFile("scripts/mongodb-workaround.js", "utf8");
    contentToPrepend = contentToPrepend.replace(/\s+/g, " ");
    contentToPrepend = comment + "\n" + contentToPrepend + "\n";

    const existingContent = await readFile(filePath, "utf8");

    if (existingContent.indexOf(comment) !== -1) {
        return console.error("error: mongo workaround already included");
    }

    const newContent = `${contentToPrepend}\n${existingContent}`;

    await writeFile(filePath, newContent);

    console.log("mongo workaround included to build");
})();
