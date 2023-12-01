<script>
    import CategoryAddPage from "../../lib/CategoryAddPage.svelte";
    import ResourcesCatalog from "./components/ResourcesCatalog.svelte";
    import miscsData from "../../../data/miscs.json";
    // import { addSuits } from "../../services/user.service";

    let items = Object.values(miscsData).map((item) => ({
        ItemType: item.uniqueName,
        info: item,
    }));

    function add(items) {
        // const postItems = items.map(({ ItemType, Configs, UpgradeVer, XP }) => ({
        //     ItemType,
        //     Configs,
        //     UpgradeVer,
        //     XP,
        // }));
        // addSuits(postItems);
    }
</script>

<CategoryAddPage {items} {add} catalogComponent={ResourcesCatalog} />
