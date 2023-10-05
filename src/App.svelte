<script>
    import { Router, Route, Link } from "svelte-routing";
    import Header from "./lib/Header.svelte";
    import Users from "./routers/users/Users.svelte";
    import Mods from "./routers/mods/Mods.svelte";
    import ModsAdd from "./routers/mods/ModsAdd.svelte";
    import Dashboard from "./routers/dashboard/Dashboard.svelte";
    import Suits from "./routers/suits/Suits.svelte";
</script>

<Router>
    <Route path="/">
        <Users />
    </Route>
    <Route path="/dashboard">
        <Header />
        <Dashboard />
    </Route>
    <Route path="/dashboard/mods">
        <Header />
        <Mods />
    </Route>

    <Route path="/dashboard/suits">
        <Header />
        <Suits />
    </Route>
    <Route path="/dashboard/mods/add">
        <Header />
        <ModsAdd />
    </Route>
</Router>
