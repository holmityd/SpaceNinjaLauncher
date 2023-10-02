<script>
    import PlayButton from "../../lib/PlayButton.svelte";
    import Search from "../../lib/Search.svelte";
    import Grid from "../../lib/Grid.svelte";
    import UserCard from "./UserCard.svelte";
    import { filterBySearch } from "../../lib/common";

    import { onMount } from "svelte";
    import { getUsers } from "../../services/user.service";

    let users = [];
    onMount(async () => {
        const response = await fetch("/api/users.json");
        // users = await response.json();
        users = await getUsers();
    });

    let searchTerm = "";
    $: filteredUsers = searchTerm
        ? filterBySearch(searchTerm, users, ["username", "email"])
        : users;
</script>

<div class="container mx-auto flex flex-col p-4">
    <div class="sticky top-0 z-10 mb-4 flex gap-4">
        <Search bind:value={searchTerm} />
        <PlayButton />
    </div>

    <Grid>
        {#each filteredUsers as user}
            <UserCard {user} />
        {/each}
    </Grid>
</div>
