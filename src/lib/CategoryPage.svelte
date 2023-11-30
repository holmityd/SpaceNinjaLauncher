<script>
    import SelectPanel from "./SelectPanel.svelte";
    import { userStore } from "../store/User.store";
    import { onMount } from "svelte";
    import CatalogActions from "./CatalogActions.svelte";

    export let modalComponent;
    export let catalogComponent;
    export let getList;
    export let remove;

    // CatalogActions
    let removeMode;

    // ModsCatalog
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
        <svelte:component
            this={catalogComponent}
            {items}
            bind:displayedItems
            on:cardClick={cardClick}
        />
    {/if}

    <SelectPanel {remove} bind:active={removeMode} bind:displayedItems bind:selectOne />
</div>

{#if modModalOpen}
    <svelte:component this={modalComponent} bind:openModal={modModalOpen} item={modModalInfo} />
{/if}
