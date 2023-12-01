<script>
    import modsData from "../../../data/mods.json";
    import ModsCatalog from "./components/ModsCatalog.svelte";
    import { addMods } from "../../services/user.service";
    import CategoryAddPage from "../../lib/CategoryAddPage.svelte";
    import ModsAddSelectPanel from "./components/ModsAddSelectPanel.svelte";
    import { maxLevelStore } from "./store/Mods.store";
    import { onMount } from "svelte";

    let items = Object.values(modsData)
        .filter((item) => item.name !== "Unfused Artifact" && item.type !== "Mod Set Mod")
        .map((item) => {
            const newItem = {
                ItemType: item.uniqueName,
                ...item,
            };
            return newItem;
        });

    function add(items) {
        const postItems = items.map(({ ItemType, ItemCount, UpgradeFingerprint }) => ({
            ItemType,
            ItemCount: $maxLevelStore ? ItemCount : ItemCount || 1,
            ...($maxLevelStore && { UpgradeFingerprint }),
        }));
        addMods(postItems);
    }

    onMount(() => {
        const maxLevelStoreSubCancel = maxLevelStore.subscribe((value) => {
            items.forEach((item) => {
                // @ts-ignore
                if (item.fusionLimit) {
                    // @ts-ignore
                    const UpgradeFingerprint = `{"lvl":${value ? item.fusionLimit : 0}}`;
                    // @ts-ignore
                    item.UpgradeFingerprint = UpgradeFingerprint;
                } else {
                    // @ts-ignore
                    item.ItemCount = 1;
                }
            });
            items = items;
        });
        return () => {
            maxLevelStoreSubCancel();
        };
    });
</script>

<CategoryAddPage
    {items}
    {add}
    catalogComponent={ModsCatalog}
    selectPanelAdditionComponent={ModsAddSelectPanel}
/>
