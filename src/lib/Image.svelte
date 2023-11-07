<script>
    import { onMount } from "svelte";
    import ImagePlaceholder from "./ImagePlaceholder.svelte";
    import { twMerge } from "tailwind-merge";
    export let src;
    export let loadingSrc = undefined;
    export let errorSrc = "https://icon-library.com/images/not-found-icon/not-found-icon-20.jpg";
    export let alt = undefined;
    export let lazy = true;

    let loaded = false;
    let failed = false;
    let loading = false;

    onMount(() => {
        const img = new Image();
        img.src = src;
        loading = true;

        img.onload = () => {
            loading = false;
            loaded = true;
        };
        img.onerror = () => {
            loading = false;
            failed = true;
        };
    });

    $: componentClass = twMerge("object-contain", $$props.class);
</script>

{#if loaded}
    <img class={componentClass} loading={lazy ? "lazy" : "eager"} {src} {alt} />
{:else if failed}
    <img class={componentClass} src={errorSrc} alt="Not Found" />
{:else if loading}
    {#if loadingSrc}
        <img
            class={componentClass}
            src="https://c.tenor.com/On7kvXhzml4AAAAi/loading-gif.gif"
            alt="Loading..."
        />
    {:else}
        <ImagePlaceholder class={componentClass} />
    {/if}
{/if}
