<script>
    import Search from "../../lib/Search.svelte";
    import { filterBySearch } from "../../lib/common";
    import categories from "./dashboard-items.json";
    import DashboardItem from "./DashboardItem.svelte";

    let searchTerm = "";

    $: filteredCategories = searchTerm
        ? filterBySearch(searchTerm, categories, ["name"])
        : categories;
</script>

<div class="container mx-auto flex flex-col p-4">
    <div class="sticky top-0 z-10 mb-4 flex gap-4">
        <Search bind:value={searchTerm} />
    </div>

    <div
        class="relative grid grid-cols-3 items-start gap-4 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-7"
    >
        {#each filteredCategories as category (category.url)}
            {#if category.working}
                <DashboardItem {category} />
            {/if}
        {/each}
    </div>
</div>
