<script>
    import CategoryPage from "../../lib/CategoryPage.svelte";
    import suitsData from "../../../data/suits.json";
    import SuitsCatalog from "./components/SuitsCatalog.svelte";
    import SuitModalInfo from "./components/SuitModalInfo.svelte";
    import { removeSuits } from "../../services/user.service";

    const getList = (value) =>
        [...value.inventory.Suits].map((suit) => ({
            ...suit,
            info: suitsData[suit.ItemType],
        }));

    function remove(items) {
        const postItems = items.map(({ _id }) => ({
            _id,
        }));
        removeSuits(postItems);
    }
</script>

<CategoryPage {getList} {remove} catalogComponent={SuitsCatalog} modalComponent={SuitModalInfo} />
