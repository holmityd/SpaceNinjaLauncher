import fs from "fs";
import path from "path";
import Items from "warframe-items";

function extractJson(fileName, items, keysToExtract) {
    const directoryPath = "../data";
    const filePath = path.join(directoryPath, `${fileName}.json`);
    const content = {};
    items.forEach((item) => {
        content[item.uniqueName] = keysToExtract.reduce(
            (obj, key) => ({ ...obj, [key]: item[key] }),
            {},
        );
    });
    // const content = items.map((item) =>
    //     keysToExtract.reduce((obj, key) => ({ ...obj, [key]: item[key] }), {}),
    // );

    if (!fs.existsSync(directoryPath)) {
        fs.mkdirSync(directoryPath, { recursive: true });
    }

    fs.writeFileSync(filePath, JSON.stringify(content), "utf8");
}

extractJson("mods", new Items({ category: ["Mods"] }), [
    "fusionLimit",
    "compatName",
    "wikiaThumbnail",
    "wikiaUrl",
    "uniqueName",
    "rarity",
    "name",
    "levelStats",
    "polarity",
    "baseDrain",
    "isPrime",
]);
