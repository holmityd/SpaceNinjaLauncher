<script>
    import modsData from "../../../data/mods.json";
    import ModsCatalog from "./components/ModsCatalog.svelte";
    import { addMods } from "../../services/user.service";
    import CategoryAddPage from "../../lib/CategoryAddPage.svelte";

    let items = Object.values(modsData)
        .filter((item) => item.name !== "Unfused Artifact" && item.type !== "Mod Set Mod")
        .map((item) => {
            const newItem = {
                ItemType: item.uniqueName,
                ...item,
            };
            // @ts-ignore
            if (item.fusionLimit) {
                // @ts-ignore
                newItem.UpgradeFingerprint = `{"lvl":${item.fusionLimit}}`;
            } else {
                // @ts-ignore
                newItem.ItemCount = 1;
            }
            return newItem;
        });

    function add(items) {
        const postItems = items.map(({ ItemType, ItemCount, UpgradeFingerprint }) => ({
            ItemType,
            ItemCount: maxLevel ? ItemCount : ItemCount || 1,
            ...(maxLevel && { UpgradeFingerprint }),
        }));
        addMods(postItems);
    }
    // TODO
    let maxLevel = false;
</script>

<CategoryAddPage {items} {add} catalogComponent={ModsCatalog} />
