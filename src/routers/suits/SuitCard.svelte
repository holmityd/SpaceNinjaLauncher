<script>
    import { Card } from "flowbite-svelte";
    import { twMerge } from "tailwind-merge";

    export let suit;

    // image
    $: image =
        suit?.wikiaThumbnail?.substring(0, suit.wikiaThumbnail.lastIndexOf(".png") + 1 + 3) ||
        "https://static.wikia.nocookie.net/warframe/images/7/72/Fusion_Core_horizontal.png";

    $: cardClass = twMerge(
        "transition overflow-hidden",
        "bg-gray-900 border-gray-800 text-gray-400",
        "hover:bg-gray-800",
        "active:ring-4",
        $$props.class,
    );
</script>

<Card
    class={cardClass}
    color="none"
    on:click={() => {
        console.log(suit);
    }}
>
    <div class="flex select-none flex-col items-center gap-4">
        <img loading="lazy" class="pointer-events-none" src={image} alt={suit.name} />
        <h5 class="text-xl font-bold tracking-tight">
            {suit.name}
        </h5>
    </div>
</Card>
