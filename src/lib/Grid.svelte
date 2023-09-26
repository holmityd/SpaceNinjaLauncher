<script>
    import { createEventDispatcher, onMount } from "svelte";

    let elRef;
    const dispatch = createEventDispatcher();
    onMount(() => {
        dispatch("elRef", elRef);
    });
</script>

<div
    bind:this={elRef}
    class="relative grid grid-cols-2 items-start gap-3 sm:grid-cols-4 md:grid-cols-4 lg:grid-cols-6
    {$$props.class || ''}"
    style={$$props.style}
>
    <slot />
</div>
