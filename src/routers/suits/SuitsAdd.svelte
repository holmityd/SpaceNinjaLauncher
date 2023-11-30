<script>
    import suitsData from "../../../data/suits.json";
    import SuitsCatalog from "./components/SuitsCatalog.svelte";
    import SelectPanel from "../../lib/SelectPanel.svelte";
    import { addSuits } from "../../services/user.service";

    // SuitsCatalog
    let displayedItems = [];
    let items = Object.values(suitsData).map((item) => ({
        ItemType: item.uniqueName,
        Configs: [],
        UpgradeVer: 101,
        XP: 0,
        info: item,
    }));
    function cardClick({ detail }) {
        selectOne(detail);
    }

    // SelectPanel
    let selectOne;
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

<div class="container mx-auto box-border flex flex-col">
    <SuitsCatalog {items} bind:displayedItems on:cardClick={cardClick} />

    <SelectPanel {add} bind:displayedItems bind:selectOne />
</div>
