<script>
    import ResourceCard from "./ResourceCard.svelte";
    import NewSearch from "../../../lib/NewSearch.svelte";
    import VirtualizedList from "../../../lib/VirtualizedList.svelte";

    export let items;
    export let displayedItems;
</script>

<NewSearch {items} bind:displayedItems />

<VirtualizedList list={displayedItems} itemComponent={ResourceCard} on:cardClick />
