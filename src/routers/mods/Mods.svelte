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
        updateGridParams();
        getAllGridItemsPosition();
        getVisibleItems();
    }
    function updateGridParams() {
        const gridRect = gridElm.getBoundingClientRect();
        const { width, height } = gridElm.firstElementChild.getBoundingClientRect();

        gridParams = {
            width,
            height,
            gridWidth: gridRect.width,
            rowItemsCount: 0,
            gap: 0,
            responsive: [],
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
        const columnElements = Math.ceil(window.innerHeight / height);
        canShowCount = columnElements * rowItemsCount * 3; // 3 - prev, current, next screen
        startingIndex = gridItemsPosition.findIndex(
            (i) => i.top + i.height * columnElements >= window.scrollY,
        );
    }
    function getAllGridItemsPosition() {
        const { rowItemsCount, height, width, gap } = gridParams;
        for (let i = 0; i < filteredRawUpgrades.length + filteredUpgrades.length; i++) {
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
        filteredRawUpgrades = filterBySearch(searchTerm, rawUpgrades, [
            "name",
            "compatName",
            "levelStats.0.stats.all",
        ]);
        filteredUpgrades = filterBySearch(searchTerm, upgrades, [
            "name",
            "compatName",
            "levelStats.0.stats.all",
        ]);
        updateGridHeight();
    }

    // lifecycle
    onMount(async () => {
        await initData();
        gridInit();

        window.addEventListener("scroll", getVisibleItems);
        window.addEventListener("resize", gridInit);
    });
    onDestroy(() => {
        window.removeEventListener("scroll", getVisibleItems);
        window.removeEventListener("resize", gridInit);
    });
</script>

<div class="sticky top-0 z-10 mb-4 flex gap-4">
    <Search bind:value={searchTerm} />
</div>

<div
    bind:this={gridElm}
    class="relative grid grid-cols-3 items-start gap-4 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-7"
    style={`height: ${gridHeight}px`}
>
    <ModCard mod={undefined} position={undefined} />
    {#each filteredRawUpgrades as mod, index (mod.uniqueName)}
        {#if startingIndex <= index && index < startingIndex + canShowCount}
            <ModCard {mod} position={gridItemsPosition[index]} />
        {/if}
    {/each}
    {#each filteredUpgrades as mod, index (mod.ItemId.$oid)}
        {#if startingIndex <= filteredRawUpgrades.length + index && filteredRawUpgrades.length + index < startingIndex + canShowCount}
            <ModCard {mod} position={gridItemsPosition[filteredRawUpgrades.length + index]} />
        {/if}
    {/each}
</div>
