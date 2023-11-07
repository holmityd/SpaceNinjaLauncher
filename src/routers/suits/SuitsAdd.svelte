<script>
    import suitsData from "../../../data/suits.json";
    import SelectPanel from "../../lib/SelectPanel.svelte";
    import { addSuits } from "../../services/user.service";
    import SuitsCatalog from "./components/SuitsCatalog.svelte";

    // SuitsCatalog
    let displayedItems = [];
    /**
     * @type {Array<import("../../types/inventory.types").SuitData>}
     */
    let suits = Object.values(suitsData).map((item) => ({
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
    <SuitsCatalog {suits} bind:displayedItems on:cardClick={cardClick} />

    <SelectPanel {add} bind:displayedItems bind:selectOne />
</div>
