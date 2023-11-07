<script>
    import PlayButton from "../../lib/PlayButton.svelte";
    import NewSearch from "../../lib/NewSearch.svelte";
    import Grid from "../../lib/Grid.svelte";
    import UserCard from "./UserCard.svelte";

    import { onMount } from "svelte";
    import { getUsers } from "../../services/user.service";

    let users = [];
    onMount(async () => {
        // const response = await fetch("/api/users.json");
        // users = await response.json();
        users = await getUsers();
    });
    let displayedItems = [];
</script>

<div class="container mx-auto box-border flex flex-col">
    <NewSearch items={users} bind:displayedItems searchTerms={["username", "email"]}>
        <PlayButton />
    </NewSearch>

    <Grid>
        {#each displayedItems as user}
            <UserCard {user} />
        {/each}
    </Grid>
</div>
