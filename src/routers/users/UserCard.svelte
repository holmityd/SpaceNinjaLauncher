<script>
    import { navigate } from "svelte-routing";
    import { Card } from "flowbite-svelte";
    import { fetchUserData } from "../../services/user.service";
    import { twMerge } from "tailwind-merge";

    export let user;

    async function userSelect() {
        navigate("/dashboard");
        fetchUserData(user);
    }

    $: cardClass = twMerge(
        "transition overflow-hidden",
        "bg-gray-900 border-gray-800 text-gray-400",
        "hover:bg-gray-800 hover:border-blue-600",
        "active:ring-4 ring-blue-600",
    );
</script>

<Card class={cardClass} padding="none" color="none" on:click={userSelect}>
    <div class="aspect-[1/1] cursor-pointer p-4">
        <h2 class="mb-2 text-2xl font-semibold">{user.display_name}</h2>
        <p>Email: {user.email}</p>
    </div>
</Card>
