<script>
    import { Router, Route, Link } from "svelte-routing";
    import Header from "./lib/Header.svelte";
    import routers from "./routers";
</script>

<Router>
    {#each routers as { path, title, component } (path)}
        <Route {path}>
            {#if title}
                <Header />
            {/if}
            <svelte:component this={component} />
        </Route>
    {/each}
</Router>
