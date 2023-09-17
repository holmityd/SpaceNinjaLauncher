<script>
  import { Link } from "svelte-routing";

  export let category;
</script>

<Link to="/category/{category.url}">
  <div class="rounded-lg bg-{category.color} p-4 text-center text-indigo-100">
    <i class="{category.icon} text-4xl"></i>
    <p class="mt-2">{category.name}</p>
  </div>
</Link>
