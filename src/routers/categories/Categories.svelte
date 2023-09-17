<script>
    import Search from "../../lib/Search.svelte";
    import { filterBySearch } from "../../lib/common";
    import categories from "./categories.json";
    import CategoryCard from "./CategoryCard.svelte";

    let searchTerm = "";

    export let id;
    id;

    $: filteredCategories = searchTerm
        ? filterBySearch(searchTerm, categories, ["name"])
        : categories;
</script>

<div class="sticky top-0 z-10 mb-4 flex gap-4">
    <Search bind:value={searchTerm} />
</div>

<div class="grid grid-cols-3 gap-4">
    {#each filteredCategories as category (category.url)}
        <CategoryCard {category} />
    {/each}
</div>
