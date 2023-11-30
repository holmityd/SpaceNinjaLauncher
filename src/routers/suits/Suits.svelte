<script>
    import { userStore } from "../../store/User.store";
    import suitsData from "../../../data/suits.json";
    import SelectPanel from "../../lib/SelectPanel.svelte";
    import SuitsCatalog from "./components/SuitsCatalog.svelte";
    import CatalogActions from "../../lib/CatalogActions.svelte";
    import SuitModalInfo from "./components/SuitModalInfo.svelte";
    import { onMount } from "svelte";
    import { removeSuits } from "../../services/user.service";

    // CatalogActions
    let removeMode;

    // SuitsCatalog
    const getList = (value) =>
        [...value.inventory.Suits].map((suit) => ({
            ...suit,
            info: suitsData[suit.ItemType],
        }));
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
        const postItems = items.map(({ _id }) => ({
            _id,
        }));
        removeSuits(postItems);
    }

    // SuitModalInfo
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
        <SuitsCatalog {items} bind:displayedItems on:cardClick={cardClick} />
    {/if}

    <SelectPanel {remove} bind:active={removeMode} bind:displayedItems bind:selectOne />
</div>

{#if modModalOpen}
    <SuitModalInfo bind:openModal={modModalOpen} item={modModalInfo} />
{/if}
