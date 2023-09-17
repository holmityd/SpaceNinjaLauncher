<script>
  import "./app.css";
  import { Router, Route, Link } from "svelte-routing";
  import Categories from "./routers/categories/Categories.svelte";
  import Users from "./routers/users/Users.svelte";
  import Mods from "./routers/mods/Mods.svelte";
</script>

<Router>
  <div class="container mx-auto p-4">
    <Route path="/" component={Users} />
    <Route path="/user/:id" let:params>
      <Categories id={params.id} />
    </Route>
    <Route path="/category/mods" component={Mods} />
  </div>
</Router>
