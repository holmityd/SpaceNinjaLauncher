<script>
    import { navigate } from "svelte-routing";
    import { Card } from "flowbite-svelte";
    import { twMerge } from "tailwind-merge";

    export let category;

    function click() {
        if (category.working) {
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

<Card class={cardClass} padding="none" color="none" on:click={click}>
    <div class="aspect-[1/1] p-4 {category.working ? 'cursor-pointer' : ''}">
        <i class="{category.icon} text-4xl"></i>
        <p>{category.name}</p>
    </div>
</Card>
