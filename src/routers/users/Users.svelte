<script>
    import PlayButton from "../../lib/PlayButton.svelte";
    import Search from "../../lib/Search.svelte";
    import UserCard from "./UserCard.svelte";
    import { filterBySearch } from "../../lib/common";

    import { onMount } from "svelte";

    let users = [];
    onMount(async () => {
        const response = await fetch("/api/users.json");
        users = await response.json();
    });

    let searchTerm = "";
    $: filteredUsers = searchTerm
        ? filterBySearch(searchTerm, users, ["username", "email"])
        : users;
</script>

<div class="sticky top-0 z-10 mb-4 flex gap-4">
    <Search bind:value={searchTerm} />
    <PlayButton />
</div>

<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5">
    {#each filteredUsers as user (user.username)}
        <UserCard {user} />
    {/each}
</div>
