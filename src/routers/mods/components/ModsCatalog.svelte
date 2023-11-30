<script>
    import NewSearch from "../../../lib/NewSearch.svelte";
    import ModCard from "./ModCard.svelte";
    import ModCategories from "./ModCategories.svelte";
    import ModFilters from "./ModFilters.svelte";
    import VirtualizedList from "../../../lib/VirtualizedList.svelte";

    export let items = [];
    export let displayedItems;

    // filter
    let filter;
    let filteredMods = [];
    $: filteredMods = (filter && filter(items)) || items;

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
</script>

<NewSearch
    class="order-1 w-1/3 flex-grow"
    items={categoriedMods}
    bind:displayedItems
    searchTerms={["name", "compatName", "levelStats.0.stats.all"]}
>
    <ModCategories class="order-3 w-full flex-grow lg:order-2" bind:value={category} />
    <ModFilters class="order-2 w-1/3 flex-grow lg:order-3" bind:value={filter} />
</NewSearch>

<VirtualizedList list={displayedItems} itemComponent={ModCard} on:cardClick />
