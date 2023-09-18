<script>
    import Icon from "@iconify/svelte";
    export let mod;
    export let position;

    const src =
        mod?.wikiaThumbnail?.substring(0, mod.wikiaThumbnail.lastIndexOf(".png") + 4) ||
        "https://static.wikia.nocookie.net/warframe/images/7/72/Fusion_Core_horizontal.png";
    const lvl = mod?.UpgradeFingerprint ? JSON.parse(mod.UpgradeFingerprint).lvl || 0 : 0;
</script>

<div
    class="rounded p-4 shadow-lg hover:cursor-pointer hover:shadow-purple-600 dark:bg-gray-900"
    style={position
        ? `position: absolute;top: ${position.top}px; left: ${position.left}px; width: ${position.width}px; height: ${position.height}px`
        : "visibility: hidden"}
>
    <div class="relative flex aspect-[4/6] items-center justify-center">
        <img {src} alt={mod?.name} loading="lazy" width="400" height="600" />
        {#if mod?.ItemCount > 1}
            <div
                class="absolute left-1 top-5 flex items-center justify-center gap-1 rounded-br rounded-tr border border-l-0 border-indigo-500 bg-black bg-opacity-75 px-2 text-sm"
            >
                <Icon icon="solar:copy-bold-duotone" />
                <b>{mod?.ItemCount}</b>
            </div>
        {/if}
        <div
            class="absolute bottom-2 left-1/2 flex w-9/12 -translate-x-1/2 transform items-center justify-center rounded-tl rounded-tr border border-b-0 border-indigo-500 bg-black bg-opacity-75 p-2"
        >
            <!-- Glowing Line -->
            <div class="drop-shadow-glow h-0.5 w-full rounded-lg bg-gray-300"></div>

            <!-- Glowing Dots -->
            <div class="absolute flex gap-1">
                {#each Array(lvl) as _}
                    <div class="drop-shadow-glow h-1.5 w-1.5 rounded-full bg-blue-300"></div>
                {/each}
                {#each Array(Math.max(0, (mod?.fusionLimit || 0) - lvl)) as _}
                    <div class="h-1.5 w-1.5 rounded-full bg-gray-300"></div>
                {/each}
            </div>
        </div>
    </div>

    <h2 class="mb-2 overflow-hidden text-ellipsis whitespace-nowrap font-semibold">{mod?.name}</h2>
    <p class="text-xs uppercase">{mod?.compatName}</p>
</div>
