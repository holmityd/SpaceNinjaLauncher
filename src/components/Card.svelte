<script>
    export let position = undefined;
    export let ring = undefined;
</script>

<!-- svelte-ignore a11y-missing-attribute -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<a
    on:click
    class="card mb-4 flex max-w-sm flex-col divide-gray-800 overflow-hidden rounded-lg border border-gray-800 bg-gray-900 text-gray-400 shadow-md transition hover:border-blue-600 hover:bg-gray-800 hover:text-gray-400 active:ring-4 active:ring-blue-800"
    style="--tw-ring-color: {ring}; {position
        ? `position: absolute;top: ${position.top}px; left: ${position.left}px; width: ${position.width}px; height: ${position.height}px`
        : 'visibility: hidden'}"
>
    <slot />
</a>

<style>
    .card:hover {
        border-color: var(--tw-ring-color);
    }
</style>
