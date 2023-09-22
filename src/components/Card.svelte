<script>
    export let position = undefined;
</script>

<div
    class="mb-4 flex max-w-sm flex-col divide-gray-800 overflow-hidden rounded-lg border border-gray-800 bg-gray-900 text-gray-400 shadow-md hover:bg-gray-800"
    style={position
        ? `position: absolute;top: ${position.top}px; left: ${position.left}px; width: ${position.width}px; height: ${position.height}px`
        : "visibility: hidden"}
>
    <slot />
</div>
