<script>
    import Grid from "../../../lib/Grid.svelte";
    import SuitCard from "./SuitCard.svelte";
    import NewSearch from "../../../lib/NewSearch.svelte";
    import VirtualizedList from "../../../lib/VirtualizedList.svelte";

    /**
     * @type {Array<import("../../../types/inventory.types").SuitData>}
     */

    export let suits;
    export let displayedItems;
</script>

<NewSearch items={suits} bind:displayedItems />

<VirtualizedList list={displayedItems} itemComponent={SuitCard} on:cardClick />

<!-- <Grid>
    {#each displayedItems as suit}
        <SuitCard {suit} on:cardClick />
    {/each}
</Grid> -->
