<script>
    import Search from "../../lib/Search.svelte";
    import { onDestroy, onMount } from "svelte";
    import { filterBySearch } from "../../lib/common";
    import modsData from "../../../data/mods.json";
    import ModCard from "./ModCard.svelte";

    // init data
    let rawUpgrades = [];
    let upgrades = [];
    async function initData() {
        const response = await fetch("/api/mods.json");
        const mods = await response.json();

        rawUpgrades = mods.RawUpgrades.map(({ ItemType, ItemCount }) => {
            const modsDataItem = modsData[ItemType];
            return modsDataItem ? { ItemCount, ...modsDataItem } : null;
        })
            .filter(Boolean)
            .sort((a, b) => b.ItemCount - a.ItemCount);
        // .filter((i, index) => index < 100);

        upgrades = mods.Upgrades.map(({ ItemType, UpgradeFingerprint, ItemId }) => {
            const modsDataItem = modsData[ItemType];
            return modsDataItem ? { ItemId, UpgradeFingerprint, ...modsDataItem } : null;
        }).filter(Boolean);
        // .filter((i, index) => index < 100);
    }

    // scroll optimization
    let gridElm;
    let gridParams;
    let gridItemsPosition = [];
    let startingIndex;
    let canShowCount;
    let gridHeight = 0;
    function gridInit() {
        for (const item of gridElm.children) {
            if (item) {
                const { width, height } = item.getBoundingClientRect();
                gridItemsPosition.push({ width, height, top: 0, left: 0 });
                break;
            }
        }
        updateGridParams();
        getAllGridItemsPosition();
        getVisibleItems();
    }
    function updateGridParams() {
        const { width, height } = gridItemsPosition[0];
        gridParams = {
            width,
            height,
            gridWidth: gridElm.offsetWidth,
            rowItemsCount: 0,
            gap: 0,
        };
        gridParams.rowItemsCount = Math.floor(gridParams.gridWidth / width);
        gridParams.gap =
            (gridParams.gridWidth - gridParams.rowItemsCount * width) /
            (gridParams.rowItemsCount - 1);
        updateGridHeight();
    }
    function updateGridHeight() {
        if (!gridParams) return;
        const { rowItemsCount, height, gap } = gridParams;
        gridHeight =
            Math.ceil((filteredRawUpgrades.length + filteredUpgrades.length) / rowItemsCount) *
                (height + gap) -
            gap;
    }
    function getVisibleItems() {
        const { height, rowItemsCount } = gridParams;
        canShowCount = Math.ceil(window.innerHeight / height) * rowItemsCount * 2;
        startingIndex = gridItemsPosition.findIndex((i) => i.top + i.height >= window.scrollY);
    }
    function getAllGridItemsPosition() {
        const { rowItemsCount, height, width, gap } = gridParams;
        for (let i = 1; i < filteredRawUpgrades.length + filteredUpgrades.length; i++) {
            const [columnIndex, rowIndex] = [Math.floor(i / rowItemsCount), i % rowItemsCount];

            gridItemsPosition[i] = {
                top: (height + gap) * columnIndex,
                left: (width + gap) * rowIndex,
                width,
                height,
            };
        }
    }

    // search
    let searchTerm = "";
    let filteredRawUpgrades = [];
    let filteredUpgrades = [];
    $: {
        filteredRawUpgrades = filterBySearch(searchTerm, rawUpgrades, ["name", "compatName"]);
        filteredUpgrades = filterBySearch(searchTerm, upgrades, ["name", "compatName"]);
        updateGridHeight();
    }

    // lifecycle
    onMount(async () => {
        await initData();
        gridInit();

        window.addEventListener("scroll", getVisibleItems);
    });
    onDestroy(() => {
        window.removeEventListener("scroll", getVisibleItems);
    });
</script>

<div class="sticky top-0 z-10 mb-4 flex gap-4">
    <Search bind:value={searchTerm} />
</div>

<div
    bind:this={gridElm}
    class="relative grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5"
    style={`height: ${gridHeight}px`}
>
    {#each filteredRawUpgrades as mod, index (mod.uniqueName)}
        {#if (startingIndex === undefined && index === 0) || (startingIndex <= index && index < startingIndex + canShowCount)}
            <ModCard {mod} {index} position={gridItemsPosition[index]} />
        {/if}
    {/each}
    {#each filteredUpgrades as mod, index (mod.ItemId.$oid)}
        {#if startingIndex <= filteredRawUpgrades.length + index && filteredRawUpgrades.length + index < startingIndex + canShowCount}
            <ModCard
                {mod}
                index={filteredRawUpgrades.length + index}
                position={gridItemsPosition[filteredRawUpgrades.length + index]}
            />
        {/if}
    {/each}
</div>
