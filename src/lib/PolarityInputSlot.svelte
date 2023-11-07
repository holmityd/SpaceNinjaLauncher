<script>
    import { Button } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    export let polarity = undefined;
    export let index;
    export let polarities;

    const polarityKeys = Object.keys(polarities);

    const nextPolarityIndex = (currentIndex = polarityKeys.indexOf(polarity)) => {
        if (currentIndex < 0 || currentIndex + 1 >= polarityKeys.length) {
            return 0;
        } else {
            currentIndex++;
            if (
                polarities[polarityKeys[currentIndex]].block &&
                polarities[polarityKeys[currentIndex]].block.indexOf(index) !== -1
            ) {
                return nextPolarityIndex(currentIndex);
            } else {
                return currentIndex;
            }
        }
    };

    const dispatch = createEventDispatcher();
    function click() {
        polarity = polarityKeys[nextPolarityIndex()];
        dispatch("change", { Slot: index, Value: polarity });
    }
</script>

<Button on:click={click} class="flex w-full flex-col bg-gray-950 hover:bg-gray-900" color="none">
    {#if polarity && polarities[polarity].img}
        <img
            class="h-6 w-6 invert"
            src={polarities[polarity].img}
            alt={polarities[polarity].name}
        />
    {/if}
    {#if !polarity || !polarities[polarity].img}
        <div class="h-6" />
    {/if}
</Button>
