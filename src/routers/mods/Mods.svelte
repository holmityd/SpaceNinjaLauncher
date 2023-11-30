<script>
    import CategoryPage from "../../lib/CategoryPage.svelte";
    import modsData from "../../../data/mods.json";
    import ModsCatalog from "./components/ModsCatalog.svelte";
    import ModModalInfo from "./components/ModModalInfo.svelte";
    import { removeMods } from "../../services/user.service";

    const getList = (value) =>
        [...value.inventory.RawUpgrades, ...value.inventory.Upgrades]
            .map((item) => ({
                ...item,
                ...modsData[item.ItemType],
            }))
            .filter((i) => !!i.uniqueName);

    function remove(items) {
        const postItems = items.map(({ ItemType, ItemCount, UpgradeFingerprint, _id }) => ({
            ItemType,
            ItemCount,
            UpgradeFingerprint,
            _id,
        }));

        removeMods(postItems);
    }
</script>

<CategoryPage {getList} {remove} catalogComponent={ModsCatalog} modalComponent={ModModalInfo} />
