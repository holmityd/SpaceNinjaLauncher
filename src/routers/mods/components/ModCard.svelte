<script>
    import { createEventDispatcher } from "svelte";
    import { twMerge } from "tailwind-merge";
    import { Card } from "flowbite-svelte";
    import Icon from "@iconify/svelte";
    export let item = undefined;

    // image
    $: image =
        item?.wikiaThumbnail?.substring(
            0,
            (item.wikiaThumbnail.lastIndexOf(".png") + 1 ||
                item.wikiaThumbnail.lastIndexOf(".gif") + 1) + 3,
        ) || "https://static.wikia.nocookie.net/warframe/images/7/72/Fusion_Core_horizontal.png";

    // level
    $: lvl = item?.UpgradeFingerprint ? JSON.parse(item.UpgradeFingerprint).lvl || 0 : 0;

    // stats
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
    $: stats =
        item?.levelStats && item.levelStats[lvl]
            ? item.levelStats[lvl]?.stats.reduce(
                  (acc, item) => acc + item.replace(regex, "") + "\n",
                  "",
              )
            : "";

    // rarity
    const rarityCard = {
        Common: "hover:border-red-400 ring-red-400",
        Uncommon: "hover:border-blue-400 ring-blue-400",
        Rare: "hover:border-yellow-200 ring-yellow-200",
        Legendary: "hover:border-white ring-white",
    };
    const rarityTitle = {
        Common: "text-red-400",
        Uncommon: "text-blue-400",
        Rare: "text-yellow-200",
        Legendary: "text-white",
    };

    // on:cardClick
    const dispatch = createEventDispatcher();
    function cardClick() {
        dispatch("cardClick", item);
    }

    $: cardClass = twMerge(
        "transition overflow-hidden",
        "bg-gray-900 border-gray-800 text-gray-400",
        "hover:bg-gray-800",
        "active:ring-4",
        rarityCard[item?.rarity],
        item?._selected && "bg-blue-800 hover:bg-blue-700",
        $$props.class,
    );
</script>

<Card class={cardClass} padding="none" color="none" on:click={cardClick}>
    <div
        class="relative flex aspect-[4/6] cursor-pointer select-none flex-col overflow-hidden text-center"
    >
        <div class="pointer-events-none aspect-[5/4] overflow-hidden rounded-t-lg bg-gray-950">
            <img class="w-full scale-110" src={image} alt="mod" />
        </div>

        <!-- count -->
        {#if item?.ItemCount > 1}
            <div
                class="pointer-events-none absolute left-0 top-0.5 flex items-center justify-center gap-1 rounded-br rounded-tr border border-l-0 border-gray-900 bg-black bg-opacity-75 px-2 text-white"
            >
                <Icon icon="solar:copy-bold-duotone" />
                <b>{item?.ItemCount}</b>
            </div>
        {/if}

        <div class="flex flex-grow flex-col overflow-hidden p-4">
            {#if item}
                <!-- name -->
                <h5 class="mb-2 text-xl font-bold tracking-tight {rarityTitle[item.rarity]}">
                    {item.name}
                </h5>

                <!-- description -->
                <div class="flex-grow overflow-hidden text-sm">
                    {#if item?.description}
                        <p>{item?.type}: {item?.description}</p>
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
                        {item?.compatName}
                    </span>
                </div>

                <!-- level -->
                <div class="mt-1 flex justify-center gap-1">
                    {#each Array(lvl) as _}
                        <div class="h-1.5 w-1.5 rounded-full bg-white"></div>
                    {/each}
                    {#each Array(Math.max(0, (item?.fusionLimit || 0) - lvl)) as _}
                        <div class="h-1.5 w-1.5 rounded-full border border-white"></div>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
</Card>
