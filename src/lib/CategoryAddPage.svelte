<script>
    import SelectPanel from "./SelectPanel.svelte";

    export let items;
    export let add;
    export let catalogComponent;
    export let selectPanelAdditionComponent = undefined;

    // ModsCatalog
    let displayedItems = [];
    function cardClick({ detail }) {
        selectOne(detail);
    }

    // SelectPanel
    let selectOne;
</script>

<div class="container mx-auto box-border flex flex-col">
    <svelte:component
        this={catalogComponent}
        {items}
        bind:displayedItems
        on:cardClick={cardClick}
    />

    <SelectPanel {add} bind:displayedItems bind:selectOne content={selectPanelAdditionComponent} />
</div>
