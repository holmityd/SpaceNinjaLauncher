<script>
    import { onMount } from "svelte";
    import NewSearch from "../../lib/NewSearch.svelte";
    import categories from "./dashboard-items.json";
    import DashboardItem from "./DashboardItem.svelte";
    let displayedItems = [];
</script>

<div class="container mx-auto box-border flex flex-col">
    <NewSearch items={categories} bind:displayedItems searchTerms={["name"]} />

    <div
        class="relative grid grid-cols-3 items-start gap-4 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-7"
    >
        {#each displayedItems as category}
            {#if category.url}
                <DashboardItem {category} />
            {/if}
        {/each}
    </div>
</div>
