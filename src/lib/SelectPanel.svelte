<script>
    import Icon from "@iconify/svelte";
    import { navigate } from "svelte-routing";
    import { Button } from "flowbite-svelte";
    import pages from "../routers/router.json";

    export let add = undefined;
    export let remove = undefined;
    export let active = true;
    export let displayedItems;
    export const selectOne = (item) => {
        if (selected.find((v) => item === v)) {
            unselect([item]);
        } else {
            select([item]);
        }
    };

    const updateDisplayedItems = (item) => {
        const index = displayedItems.indexOf(item);
        if (index !== -1) displayedItems[index] = item;
    };

    let selected = [];
    function select(items) {
        const notExistingItems = items.filter(
            (item) => !selected.find((select) => select === item),
        );
        selected = [...selected, ...notExistingItems];

        items.forEach((item) => {
            item._selected = true;
            updateDisplayedItems(item);
        });
    }
    function unselect(items) {
        selected = selected.filter((select) => !items.find((item) => select === item));

        items.forEach((item) => {
            item._selected = false;
            updateDisplayedItems(item);
        });
    }

    $: {
        if (!active && selected.length) {
            clear();
        }
    }

    // buttons
    function selectAllDisplayed() {
        select(displayedItems);
    }
    function clear() {
        unselect(selected);
    }
    function removeAction() {
        if (selected.length) {
            remove(selected);
        }

        clear();

        active = false;
    }
    function addAction() {
        if (selected.length) {
            add(selected);
        }
        clear();

        const page = pages.find((i) => i.path === window.location.pathname);

        navigate(page.back);
    }
</script>

{#if active}
    <div class=" mt-20"></div>
    <div class="box-b container fixed bottom-0 flex flex-row items-center gap-4 bg-gray-800 p-4">
        <p class="flex-grow">Selected {selected.length} items</p>
        <Button
            class="gap-1 border border-transparent hover:border-blue-600"
            on:click={selectAllDisplayed}
        >
            <Icon icon="iconoir:list-select" /> Select all
        </Button>
        <Button class="gap-1 border border-transparent hover:border-blue-600" on:click={clear}>
            <Icon icon="mdi:broom" /> Clear
        </Button>
        {#if remove}
            <Button
                class="gap-1 border border-transparent hover:border-red-600"
                color="red"
                on:click={removeAction}><Icon icon="mdi:delete" /> Remove</Button
            >
        {/if}
        {#if add}
            <Button
                class="gap-1 border border-transparent hover:border-green-600"
                color="green"
                on:click={addAction}><Icon icon="mdi:add" /> Add</Button
            >
        {/if}
    </div>
{/if}
