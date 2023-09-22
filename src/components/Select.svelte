<script>
    export let value;
    export let classes = "";
    export let options = [];
    export let id = undefined;
</script>

<select
    {id}
    bind:value
    on:change
    class="block w-full rounded-lg border border-gray-800 bg-gray-900 p-2.5 text-sm text-white placeholder-gray-400 outline-none transition-colors hover:border-blue-600 hover:bg-gray-800 focus:bg-gray-800 focus:ring-4 focus:ring-blue-800 {classes}"
>
    {#each options as option}
        <option value={option}>{option}</option>
    {/each}
</select>
