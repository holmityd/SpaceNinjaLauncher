<script>
    import Search from "../../../lib/Search.svelte";
    import { onMount } from "svelte";
    import { filterBySearch } from "../../../lib/common";
    import ModCard from "./ModCard.svelte";
    import ModCategories from "./ModCategories.svelte";
    import ModFilters from "./ModFilters.svelte";
    import { createEventDispatcher } from "svelte";
    import VirtualizedList from "../../../lib/VirtualizedList.svelte";

    export let mods = [];

    // filter
    let filter;
    let filteredMods = [];
    $: filteredMods = (filter && filter(mods)) || mods;

    // category
    let category;
    let categoriedMods = [];
    $: {
        switch (category?.key) {
            case undefined:
            case "all":
                categoriedMods = filteredMods;
                break;
            case "duplicates":
                categoriedMods = filteredMods.filter((item) => item.ItemCount > 1);
                break;
            case "exilus":
                categoriedMods = filteredMods.filter((item) => item.isUtility);
                break;
            default:
                categoriedMods = filteredMods.filter((item) =>
                    category?.compatNames.includes(item.compatName?.toLowerCase()),
                );
        }
    }

    // search
    const dispatch = createEventDispatcher();
    let searchTerm = "";
    let searchedMods = [];
    $: {
        searchedMods = filterBySearch(searchTerm, categoriedMods, [
            "name",
            "compatName",
            "levelStats.0.stats.all",
        ]);
        dispatch("displayedListChange", searchedMods);
    }

    // lifecycle
    onMount(async () => {
        dispatch("displayedListChange", searchedMods);
    });
</script>

<div
    class="sticky top-0 z-10 flex flex-wrap gap-4 py-4 lg:flex-nowrap"
    style="background-color: #2f2f2f;"
>
    <Search
        class="order-1 w-1/3 flex-grow"
        id="warframelauncher-mod-search"
        bind:value={searchTerm}
    />
    <ModCategories class="order-3 w-full flex-grow lg:order-2" bind:value={category} />
    <ModFilters class="order-2 w-1/3 flex-grow lg:order-3" bind:value={filter} />
</div>

<VirtualizedList list={searchedMods} itemComponent={ModCard} on:cardClick />
