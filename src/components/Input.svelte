<script>
    export let value = "";
    export let placeholder = "";
</script>

<input
    class="focus:border-primary-500 focus:ring-primary-500 block w-full rounded-lg border-gray-800 bg-gray-900 p-2.5 text-sm text-white placeholder-gray-400 disabled:cursor-not-allowed disabled:opacity-50"
    {placeholder}
    type="text"
    bind:value
/>
