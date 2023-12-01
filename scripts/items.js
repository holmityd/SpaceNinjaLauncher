import fs from "fs";
import path from "path";
import Items from "warframe-items";

function getKeys(items) {
    let itemsKey = new Set();
    items.forEach((item) => {
        Object.keys(item).forEach((key) => {
            itemsKey.add(key);
        });
    });
    itemsKey = new Set([...itemsKey].sort((a, b) => a.localeCompare(b)));
    console.log(itemsKey);
}

function extractJson(fileName, items, keysToExtract = undefined) {
    const directoryPath = "data";
    const filePath = path.join(directoryPath, `${fileName}.json`);
    let content = items;
    if (keysToExtract) {
        content = {};
        items.forEach((item) => {
            content[item.uniqueName] = keysToExtract.reduce(
                (obj, key) => ({ ...obj, [key]: item[key] }),
                {},
            );
        });
    }

    if (!fs.existsSync(directoryPath)) {
        fs.mkdirSync(directoryPath, { recursive: true });
    }

    fs.writeFileSync(filePath, JSON.stringify(content), "utf8");
}

// getKeys(new Items({ category: ["Warframes"] }));

extractJson("mods", new Items({ category: ["Mods"] }), [
    "fusionLimit",
    "compatName",
    "wikiaThumbnail",
    "wikiaUrl",
    "uniqueName",
    "rarity",
    "description",
    "type",
    "isUtility",
    "name",
    "levelStats",
    "polarity",
    "baseDrain",
    "isPrime",
]);

extractJson("suits", new Items({ category: ["Warframes"] }), [
    "abilities",
    "armor",
    "aura",
    "bpCost",
    "buildPrice",
    "buildQuantity",
    "buildTime",
    "category",
    "color",
    "components",
    "conclave",
    "consumeOnBuild",
    "description",
    "estimatedVaultDate",
    "exalted",
    "health",
    "imageName",
    "isPrime",
    "marketCost",
    "masterable",
    "masteryReq",
    "name",
    "passiveDescription",
    "polarities",
    "power",
    "productCategory",
    "releaseDate",
    "sex",
    "shield",
    "skipBuildTimePrice",
    "sprint",
    "sprintSpeed",
    "stamina",
    "tradable",
    "type",
    "uniqueName",
    "vaultDate",
    "vaulted",
    "wikiaThumbnail",
    "wikiaUrl",
]);

extractJson("miscs", new Items({ category: ["Misc"] }));
