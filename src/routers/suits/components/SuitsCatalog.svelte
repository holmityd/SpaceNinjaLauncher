<script>
    import SuitCard from "./SuitCard.svelte";
    import NewSearch from "../../../lib/NewSearch.svelte";
    import VirtualizedList from "../../../lib/VirtualizedList.svelte";

    /**
     * @type {Array<import("../../../types/inventory.types").SuitData>}
     */

    export let items;
    export let displayedItems;
</script>

<NewSearch {items} bind:displayedItems />

<VirtualizedList list={displayedItems} itemComponent={SuitCard} on:cardClick />
