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
    const getSuits = (value) =>
        [...value.inventory.Suits].map((suit) => ({
            ...suit,
            info: suitsData[suit.ItemType],
        }));
    let displayedItems = [];
    /**
     * @type {Array<import("../../types/inventory.types").SuitData>}
     */
    let suits;
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
        const stopModSubsciption = userStore.subscribe((value) => {
            suits = getSuits(value);
        });
        return () => {
            stopModSubsciption();
        };
    });
</script>

{#if suits}
    <div class="container mx-auto box-border flex flex-col">
        <CatalogActions bind:removeMode />

        <SuitsCatalog {suits} bind:displayedItems on:cardClick={cardClick} />

        <SelectPanel {remove} bind:active={removeMode} bind:displayedItems bind:selectOne />
    </div>
{/if}
{#if modModalInfo}
    <SuitModalInfo bind:openModal={modModalOpen} suit={modModalInfo} />
{/if}
