<script>
    import Icon from "@iconify/svelte";
    import { Tooltip, Button, ButtonGroup } from "flowbite-svelte";
    import categories from "./mod-categories.json";

    // bind:value
    export let value;
    function categorySelect({ key, compatNames }) {
        value = { key, compatNames };
    }

    // tooltip
    let tooltip;
    function updateTooltip({ target }) {
        if (target instanceof HTMLElement) {
            tooltip = target.dataset.tooltip;
        }
    }
</script>

<ButtonGroup class={$$props.class}>
    {#each categories as category}
        <Button
            size="xs"
            color={category.key === (value?.key || "all") ? "blue" : "primary"}
            class="category-button h-11 flex-grow gap-1 border border-transparent capitalize hover:border-blue-600"
            data-tooltip={category.key}
            on:click={() => categorySelect(category)}
        >
            <Icon class="text-lg" icon={category.icon} />
        </Button>
    {/each}
</ButtonGroup>
<Tooltip
    type="custom"
    arrow={false}
    class="bg-gray-950"
    triggeredBy=".category-button"
    on:show={updateTooltip}
>
    {tooltip}
</Tooltip>
