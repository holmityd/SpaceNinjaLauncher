<script>
    import { Search } from "flowbite-svelte";
    import { filterBySearch } from "./common";
    import { twMerge } from "tailwind-merge";
    export let id = undefined;
    export let searchTerms = ["info.name"];
    export let items;
    export let displayedItems;

    let search = "";
    $: displayedItems = filterBySearch(search, items, searchTerms);
    $: wrapperClass = twMerge(["w-full", $$props.class]);
</script>

<div
    class="sticky top-0 z-10 mt-1 flex flex-wrap gap-3 py-2 lg:flex-nowrap"
    style="background-color: #2f2f2f;"
>
    <div class={wrapperClass}>
        <Search
            class="h-11 dark:border-gray-800 dark:bg-gray-900"
            {id}
            bind:value={search}
            placeholder="Search..."
        />
    </div>

    <slot />
</div>
