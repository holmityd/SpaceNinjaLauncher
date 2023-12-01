<script>
    import CategoryPage from "../../lib/CategoryPage.svelte";
    import ResourcesCatalog from "./components/ResourcesCatalog.svelte";
    import ResourceModalInfo from "./components/ResourceModalInfo.svelte";
    // import suitsData from "../../../data/suits.json";
    // import { removeSuits } from "../../services/user.service";

    const getList = (value) => [];
    // [...value.inventory.Suits].map((suit) => ({
    //     ...suit,
    //     info: suitsData[suit.ItemType],
    // }));

    function remove(items) {
        // const postItems = items.map(({ _id }) => ({
        //     _id,
        // }));
        // removeSuits(postItems);
    }
</script>

<CategoryPage
    {getList}
    {remove}
    catalogComponent={ResourcesCatalog}
    modalComponent={ResourceModalInfo}
/>
