<script>
    export let value;
    export let classes = "";
    export let options = [];
</script>

<select
    bind:value
    on:change
    class="focus:ring-primary-700 focus:border-primary-700 block w-full rounded-lg border border-gray-800 bg-gray-900 p-2.5 text-sm text-white placeholder-gray-400 {classes}"
>
    {#each options as option}
        <option value={option}>{option}</option>
    {/each}
</select>
