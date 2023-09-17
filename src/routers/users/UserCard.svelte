<script>
  import { Link } from "svelte-routing";

  export let user;
</script>

<Link to="/user/{user.id}">
  <div class="rounded p-4 shadow-lg hover:cursor-pointer hover:shadow-purple-600 dark:bg-gray-900">
    <h2 class="mb-2 text-2xl font-semibold">{user.username}</h2>
    <p>Email: {user.email}</p>
    <p>Level: {user.level}</p>
  </div>
</Link>
