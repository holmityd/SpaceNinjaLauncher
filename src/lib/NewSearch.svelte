<script>
    import { Search } from "flowbite-svelte";
    import { filterBySearch } from "./common";
    export let id = undefined;
    export let searchTerms = ["info.name"];
    export let items;
    export let displayedItems;

    let search = "";
    $: displayedItems = filterBySearch(search, items, searchTerms);
</script>

<div class="sticky top-0 z-10 mt-1 flex flex-nowrap gap-3 py-2" style="background-color: #2f2f2f;">
    <Search
        class="dark:border-gray-800 dark:bg-gray-900 {$$props.class}"
        {id}
        bind:value={search}
        placeholder="Search..."
    />

    <slot />
</div>
