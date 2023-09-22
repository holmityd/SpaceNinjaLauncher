<script>
    export let value = "";
    export let placeholder = "";
    export let id = undefined;
</script>

<input
    {id}
    class="block w-full rounded-lg border-gray-800 bg-gray-900 p-2.5 text-sm text-white placeholder-gray-400 transition-colors hover:border-blue-600 hover:bg-gray-800 focus:bg-gray-800 focus:outline-4 focus:ring-4 focus:ring-blue-800 disabled:cursor-not-allowed disabled:opacity-50"
    {placeholder}
    type="text"
    bind:value
/>
