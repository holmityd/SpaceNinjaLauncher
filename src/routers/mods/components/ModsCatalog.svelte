<script>
    import Search from "../../../lib/Search.svelte";
    import { onDestroy, onMount } from "svelte";
    import { filterBySearch } from "../../../lib/common";
    import ModCard from "./ModCard.svelte";
    import ModCategories from "./ModCategories.svelte";
    import ModFilters from "./ModFilters.svelte";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let mods = [];

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
        gridHeight = Math.ceil(searchedMods.length / rowItemsCount) * (height + gap) - gap;
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
        for (let i = 0; i < searchedMods.length; i++) {
            const [columnIndex, rowIndex] = [Math.floor(i / rowItemsCount), i % rowItemsCount];

            gridItemsPosition[i] = {
                top: (height + gap) * columnIndex,
                left: (width + gap) * rowIndex,
                width,
                height,
            };
        }
    }

    // filter
    let filter;
    let filteredMods = [];
    $: {
        filteredMods = (filter && filter(mods)) || mods;
    }

    // category
    let category;
    let categoriedMods = [];
    $: {
        switch (category?.value) {
            case undefined:
            case "all":
                categoriedMods = filteredMods;
                break;
            case "duplicates":
                categoriedMods = filteredMods.filter((item) => item.ItemCount > 1);
                break;
            case "exilus":
                // console.log(
                //     filteredMods.filter((item) => {
                //         console.log(item.isUtility);
                //         return item.isUtility;
                //     }),
                // );
                categoriedMods = filteredMods.filter((item) => item.isUtility);
                break;
            default:
                categoriedMods = filteredMods.filter((item) =>
                    category?.copmatNames.includes(item.compatName?.toLowerCase()),
                );
        }
    }

    // search
    let searchTerm = "";
    let searchedMods = [];
    $: {
        searchedMods = filterBySearch(searchTerm, categoriedMods, [
            "name",
            "compatName",
            "levelStats.0.stats.all",
        ]);
        updateGridHeight();
        dispatch("displayedListChange", searchedMods);
    }

    // lifecycle
    onMount(async () => {
        gridInit();
        dispatch("displayedListChange", searchedMods);

        window.addEventListener("scroll", getVisibleItems);
        window.addEventListener("resize", gridInit);
    });
    onDestroy(() => {
        window.removeEventListener("scroll", getVisibleItems);
        window.removeEventListener("resize", gridInit);
    });
</script>

<div class="sticky top-0 z-10 flex gap-4 py-4" style="background-color: #2f2f2f;">
    <Search id="warframelauncher-mod-search" bind:value={searchTerm} />
    <ModCategories bind:value={category} />
    <ModFilters bind:value={filter} />
</div>

<div
    bind:this={gridElm}
    class="relative grid grid-cols-3 items-start gap-4 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-7"
    style={`height: ${gridHeight}px`}
>
    <ModCard mod={undefined} />
    {#each searchedMods as mod, index}
        {#if startingIndex <= index && index < startingIndex + canShowCount}
            <ModCard {mod} position={gridItemsPosition[index]} on:cardClick />
        {/if}
    {/each}
</div>
