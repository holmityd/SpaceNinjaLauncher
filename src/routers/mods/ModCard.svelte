<script>
    import Card from "../../components/Card.svelte";
    import Icon from "@iconify/svelte";
    export let mod;
    export let position = undefined;

    const statIcons = [
        "<DT_IMPACT>",
        "<DT_FREEZE>",
        "<DT_FIRE>",
        "<DT_POISON>",
        "<DT_SLASH>",
        "<DT_ELECTRICITY>",
        "<DT_PUNCTURE>",
        "<DT_RADIATION>",
        "<DT_SENTIENT>",
        "<DT_EXPLOSION>",
        "<DT_CORROSIVE>",
        "<DT_MAGNETIC>",
        "<DT_VIRAL>",
    ];
    const regexPattern = `(${statIcons.join("|")})`;
    const regex = new RegExp(regexPattern, "g");
    const rarityColors = {
        Legendary: "#fff",
        Uncommon: "#67e8f9",
        Common: "#fca5a5",
        Rare: "#fcd34d",
    };

    $: image =
        mod?.wikiaThumbnail?.substring(0, mod.wikiaThumbnail.lastIndexOf(".png") + 4) ||
        "https://static.wikia.nocookie.net/warframe/images/7/72/Fusion_Core_horizontal.png";

    $: lvl = mod?.UpgradeFingerprint ? JSON.parse(mod.UpgradeFingerprint).lvl || 0 : 0;

    $: stats =
        mod?.levelStats && mod.levelStats[lvl]
            ? mod.levelStats[lvl]?.stats.reduce(
                  (acc, item) => acc + item.replace(regex, "") + "\n",
                  "",
              )
            : "";
</script>

<Card {position} ring={rarityColors[mod?.rarity]}>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
        class="relative flex aspect-[4/6] cursor-pointer select-none flex-col overflow-hidden"
        on:click={() => console.log(mod)}
    >
        <div class="pointer-events-none aspect-[5/4] overflow-hidden rounded-t-lg bg-gray-950">
            <img class="w-full scale-110" src={image} alt="mod" />
        </div>

        <!-- count -->
        {#if mod?.ItemCount > 1}
            <div
                class="pointer-events-none absolute left-0 top-0.5 flex items-center justify-center gap-1 rounded-br rounded-tr border border-l-0 border-gray-900 bg-black bg-opacity-75 px-2 text-white"
            >
                <Icon icon="solar:copy-bold-duotone" />
                <b>{mod?.ItemCount}</b>
            </div>
        {/if}

        <div class="flex flex-grow flex-col overflow-hidden p-4">
            {#if mod}
                <!-- name -->
                <h5
                    class="mb-2 text-xl font-bold tracking-tight"
                    style="color: {rarityColors[mod?.rarity]}"
                >
                    {mod.name}
                </h5>

                <!-- description -->
                <div class="flex-grow overflow-hidden text-sm">
                    {#if mod?.description}
                        <p>{mod?.type}: {mod?.description}</p>
                    {/if}

                    <!-- stats -->
                    {#if stats}
                        <p class="mb-3 whitespace-pre-line font-normal leading-tight text-gray-400">
                            {stats}
                        </p>
                    {/if}
                </div>

                <!-- compat -->
                <div>
                    <span
                        class="m-1 inline-block rounded border border-gray-300 px-3 py-1 text-xs uppercase"
                    >
                        {mod?.compatName}
                    </span>
                </div>

                <!-- level -->
                <div class="mt-1 flex justify-center gap-1">
                    {#each Array(lvl) as _}
                        <div class="h-1.5 w-1.5 rounded-full bg-white"></div>
                    {/each}
                    {#each Array(Math.max(0, (mod?.fusionLimit || 0) - lvl)) as _}
                        <div class="h-1.5 w-1.5 rounded-full border border-white"></div>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
</Card>
