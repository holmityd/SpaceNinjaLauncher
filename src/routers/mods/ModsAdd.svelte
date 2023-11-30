<script>
    import modsData from "../../../data/mods.json";
    import ModsCatalog from "./components/ModsCatalog.svelte";
    import SelectPanel from "../../lib/SelectPanel.svelte";
    import { addMods } from "../../services/user.service";

    // ModsCatalog
    let displayedItems = [];
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
    function cardClick({ detail }) {
        selectOne(detail);
    }

    // SelectPanel
    let selectOne;
    function add(items) {
        const postItems = items.map(({ ItemType, ItemCount, UpgradeFingerprint }) => ({
            ItemType,
            ItemCount: maxLevel ? ItemCount : ItemCount || 1,
            ...(maxLevel && { UpgradeFingerprint }),
        }));
        addMods(postItems);
    }
    let maxLevel = false;
</script>

<div class="container mx-auto box-border flex flex-col">
    <ModsCatalog {items} bind:displayedItems on:cardClick={cardClick} />

    <SelectPanel {add} bind:displayedItems bind:selectOne />
</div>
