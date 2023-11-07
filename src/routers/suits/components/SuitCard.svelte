<script>
    import { Card } from "flowbite-svelte";
    import { twMerge } from "tailwind-merge";
    import Image from "../../../lib/Image.svelte";
    import { createEventDispatcher } from "svelte";
    import { getSuitLvlByXP } from "../services/suit.service";

    /**
     * @type {import("../../../types/inventory.types").SuitData}
     */
    export let suit;

    // image
    $: image = suit?.info.wikiaThumbnail?.substring(
        0,
        suit.info.wikiaThumbnail.lastIndexOf(".png") + 1 + 3,
    );

    $: cardClass = twMerge(
        "transition overflow-hidden",
        "bg-gray-900 border-gray-800 text-gray-400",
        "hover:bg-gray-800",
        "active:ring-4",
        suit._selected && "bg-blue-800 hover:bg-blue-700",
        $$props.class,
    );

    const dispatch = createEventDispatcher();
    function cardClick() {
        dispatch("cardClick", suit);
    }

    let lvl = 0;
    $: {
        if (suit.XP != undefined) {
            lvl = getSuitLvlByXP(suit.XP);
        }
    }
</script>

<Card class={cardClass} color="none" on:click={cardClick}>
    <div class="flex select-none flex-col items-center gap-4">
        <p>({suit.Polarized | 0})</p>
        <Image class="pointer-events-none h-32 w-32" src={image} alt={suit.info.name} />
        <h5
            class="overflow-hidden text-ellipsis whitespace-nowrap text-xl font-bold tracking-tight"
        >
            {suit.info.name} [{lvl}]
        </h5>
    </div>
</Card>
