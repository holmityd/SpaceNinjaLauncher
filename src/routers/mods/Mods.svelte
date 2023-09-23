<script>
    import { onMount } from "svelte";
    import modsData from "../../../data/mods.json";
    import ModsCatalog from "./components/ModsCatalog.svelte";

    let mods;

    async function getMods() {
        const modsResponse = await fetch("/api/mods.json");
        const modsResponseJson = await modsResponse.json();

        const rawUpgrades = modsResponseJson.RawUpgrades.map(({ ItemType, ItemCount }) => {
            const modsDataItem = modsData[ItemType];
            return modsDataItem ? { ItemCount, ...modsDataItem } : null;
        }).filter(Boolean);

        const upgrades = modsResponseJson.Upgrades.map(
            ({ ItemType, UpgradeFingerprint, ItemId }) => {
                const modsDataItem = modsData[ItemType];
                return modsDataItem ? { ItemId, UpgradeFingerprint, ...modsDataItem } : null;
            },
        ).filter(Boolean);

        return rawUpgrades.concat(upgrades);
    }

    onMount(async () => {
        mods = await getMods();
    });
</script>

{#if mods}
    <ModsCatalog {mods} />
{/if}
