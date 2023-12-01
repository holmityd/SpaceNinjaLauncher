<script>
    import { Card } from "flowbite-svelte";
    import { twMerge } from "tailwind-merge";
    import { createEventDispatcher } from "svelte";

    export let item = undefined;

    // image
    $: image = item?.info.wikiaThumbnail?.substring(
        0,
        item.info.wikiaThumbnail.lastIndexOf(".png") + 1 + 3,
    );

    const dispatch = createEventDispatcher();
    function cardClick() {
        dispatch("cardClick", item);
    }

    $: cardClass = twMerge(
        "transition overflow-hidden",
        "bg-gray-900 border-gray-800 text-gray-400",
        "hover:bg-gray-800",
        "active:ring-4",
        "cursor-pointer",
        item?._selected && "bg-blue-800 hover:bg-blue-700",
        $$props.class,
    );
</script>

<Card class={cardClass} color="none" on:click={cardClick}>
    <div class="flex select-none flex-col items-center gap-4">
        <div class="pointer-events-none h-32 w-32">
            <img src={image} alt={item?.info.name} />
        </div>
        <h5
            class="overflow-hidden text-ellipsis whitespace-nowrap text-xl font-bold tracking-tight"
        >
            {item?.info.name}
        </h5>
    </div>
</Card>
