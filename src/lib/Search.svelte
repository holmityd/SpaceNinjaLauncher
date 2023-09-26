<script>
    import { Input } from "flowbite-svelte";
    export let value = "";
    export let id = undefined;
</script>

<Input
    class="dark:border-gray-800 dark:bg-gray-900 {$$props.class}"
    {id}
    bind:value
    placeholder="Search..."
/>
