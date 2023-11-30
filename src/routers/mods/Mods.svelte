<script>
    import modsData from "../../../data/mods.json";
    import ModsCatalog from "./components/ModsCatalog.svelte";
    import SelectPanel from "../../lib/SelectPanel.svelte";
    import { userStore } from "../../store/User.store";
    import { removeMods } from "../../services/user.service";
    import { onMount } from "svelte";
    import ModModalInfo from "./components/ModModalInfo.svelte";
    import CatalogActions from "../../lib/CatalogActions.svelte";

    // CatalogActions
    let removeMode;

    // ModsCatalog
    const getList = (value) =>
        [...value.inventory.RawUpgrades, ...value.inventory.Upgrades]
            .map((item) => ({
                ...item,
                ...modsData[item.ItemType],
            }))
            .filter((i) => !!i.uniqueName);
    let items;
    let displayedItems = [];
    function cardClick({ detail }) {
        if (!removeMode) {
            modModalOpen = true;
            modModalInfo = detail;
        } else {
            selectOne(detail);
        }
    }

    // SelectPanel
    let selectOne;
    function remove(items) {
        const postItems = items.map(({ ItemType, ItemCount, UpgradeFingerprint, _id }) => ({
            ItemType,
            ItemCount,
            UpgradeFingerprint,
            _id,
        }));

        removeMods(postItems);
    }

    // ModModalInfo
    let modModalOpen = false;
    let modModalInfo;

    onMount(() => {
        setTimeout(() => {
            displayedItems = [...displayedItems]; // reactivity fix after NewSearch will give displayedItems
        });
        const stopGetListSubscription = userStore.subscribe((value) => {
            items = getList(value);
        });
        return () => {
            stopGetListSubscription();
        };
    });
</script>

<div class="container mx-auto box-border flex flex-col">
    <CatalogActions bind:removeMode />

    {#if items}
        <ModsCatalog {items} bind:displayedItems on:cardClick={cardClick} />
    {/if}

    <SelectPanel {remove} bind:active={removeMode} bind:displayedItems bind:selectOne />
</div>

{#if modModalOpen}
    <ModModalInfo bind:openModal={modModalOpen} item={modModalInfo} />
{/if}
