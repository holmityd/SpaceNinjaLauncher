<script>
    import { userStore } from "../../store/User.store";

    import suitsData from "../../../data/suits.json";
    import Grid from "../../lib/Grid.svelte";
    import { filterBySearch } from "../../lib/common";
    import SuitCard from "./SuitCard.svelte";
    import Search from "../../lib/Search.svelte";

    let suits = $userStore.inventory.Suits.map((suit) => ({
        ...suit,
        ...suitsData[suit.ItemType],
    }));

    let searchTerm = "";
    let searchedSuits = [];
    $: searchedSuits = filterBySearch(searchTerm, suits, ["name"]);
</script>

<div class="container mx-auto box-border flex flex-col">
    <div
        class="sticky top-0 z-10 flex flex-wrap gap-4 py-4 lg:flex-nowrap"
        style="background-color: #2f2f2f;"
    >
        <Search bind:value={searchTerm} />
    </div>

    <Grid>
        {#each searchedSuits as suit, index}
            <SuitCard {suit} />
        {/each}
    </Grid>
</div>
