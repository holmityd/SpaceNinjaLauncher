<script>
    import suitsData from "../../../data/suits.json";
    import SuitsCatalog from "./components/SuitsCatalog.svelte";
    import { addSuits } from "../../services/user.service";
    import CategoryAddPage from "../../lib/CategoryAddPage.svelte";

    let items = Object.values(suitsData).map((item) => ({
        ItemType: item.uniqueName,
        Configs: [],
        UpgradeVer: 101,
        XP: 0,
        info: item,
    }));

    function add(items) {
        const postItems = items.map(({ ItemType, Configs, UpgradeVer, XP }) => ({
            ItemType,
            Configs,
            UpgradeVer,
            XP,
        }));
        addSuits(postItems);
    }
</script>

<CategoryAddPage {items} {add} catalogComponent={SuitsCatalog} />
