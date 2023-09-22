<script>
    export let active = false;
    export let classes = "";
    export let roundedFull = false;
</script>

<button
    on:click
    class="inline-flex items-center justify-center px-5 py-2.5 text-center text-sm font-medium text-white focus:outline-none focus:ring-4
        {classes} 
        {roundedFull ? 'rounded-full' : 'rounded-lg'} 
        {active
        ? // :'bg-purple-600 hover:bg-purple-700 focus:ring-purple-900'
          'bg-blue-600 hover:bg-blue-700 focus:ring-blue-800'
        : 'bg-gray-900 hover:bg-gray-800 focus:ring-gray-800'}"
>
    <slot />
</button>
