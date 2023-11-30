<script>
    import { navigate } from "svelte-routing";
    import { Card } from "flowbite-svelte";
    import { twMerge } from "tailwind-merge";

    export let category;

    function navigateToCategory() {
        if (category?.url) {
            navigate(`/dashboard/${category.url}`);
        }
    }

    $: cardClass = twMerge(
        "transition overflow-hidden",
        "bg-gray-900 border-gray-800 text-gray-400",
        "hover:bg-gray-800 hover:border-blue-600",
        "active:ring-4 ring-blue-600",
    );
</script>

<Card class={cardClass} padding="none" color="none" on:click={navigateToCategory}>
    <div class="group relative aspect-[1/1] cursor-pointer select-none p-4">
        <div class="aspect-[1/1] w-full overflow-hidden">
            <div class="relative -mt-1 aspect-[1/1] w-full overflow-hidden">
                <img
                    class="pointer-events-none absolute w-full group-hover:bottom-0"
                    src={category.img || "default-image.jpg"}
                    alt={category.name}
                />
            </div>
        </div>

        <h5
            class="border-box absolute bottom-0 left-0 w-full bg-gray-900 px-4 py-2 text-center capitalize tracking-wide group-hover:bg-gray-800"
        >
            {category.name}
        </h5>
    </div>
</Card>
