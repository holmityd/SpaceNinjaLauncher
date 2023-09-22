<script>
    import Select from "../../components/Select.svelte";
    export let value;
    let selectedOption = "recent";

    const rarityOrder = ["Legendary", "Rare", "Uncommon", "Common"];

    const filters = {
        recent: (arr) => arr,
        rarity: (arr) =>
            arr
                .slice()
                .sort((a, b) => rarityOrder.indexOf(a.rarity) - rarityOrder.indexOf(b.rarity)),
        name: (arr) => arr.slice().sort((a, b) => a.name?.localeCompare(b.name)),
        polarity: (arr) => arr.slice().sort((a, b) => a.polarity?.localeCompare(b.polarity)),
        type: (arr) => arr.slice().sort((a, b) => a.compatName?.localeCompare(b.compatName)),
        rank: (arr) =>
            arr.slice().sort((a, b) => {
                const alvl = a?.UpgradeFingerprint ? JSON.parse(a.UpgradeFingerprint).lvl || 0 : 0;
                const blvl = b?.UpgradeFingerprint ? JSON.parse(b.UpgradeFingerprint).lvl || 0 : 0;
                return blvl - alvl;
            }),
        duplicates: (arr) => arr.slice().sort((a, b) => b.ItemCount - a.ItemCount),
    };
    function handleChange(event) {
        value = filters[event.target.value];
    }
</script>

<Select
    id="warframelauncher-mod-filter"
    classes="w-32 capitalize"
    bind:value={selectedOption}
    options={Object.keys(filters)}
    on:change={handleChange}
/>
