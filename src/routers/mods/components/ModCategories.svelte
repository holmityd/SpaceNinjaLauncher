<script>
    import Button from "../../../components/Button.svelte";
    import categories from "./mod-categories.json";

    export let value;

    // // output
    // import { createEventDispatcher } from "svelte";
    // const dispatch = createEventDispatcher();
    function categorySelect(category) {
        categoryValue = category;
        value = { value: category, copmatNames: categories[category] };
        // dispatch("categorySelect", { value: category, copmatNames: categories[category] });
    }

    let categoryValue = "all";
</script>

<div class="flex gap-4">
    {#each Object.keys(categories) as category}
        <Button
            roundedFull={true}
            active={category === categoryValue}
            classes="capitalize"
            on:click={() => categorySelect(category)}>{category}</Button
        >
    {/each}
</div>
