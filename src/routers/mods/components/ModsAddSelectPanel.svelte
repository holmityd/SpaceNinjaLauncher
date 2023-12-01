<script>
    import { Checkbox } from "flowbite-svelte";
    import { maxLevelStore } from "../store/Mods.store";

    function onChange({ target }) {
        const { checked } = target;
        maxLevelStore.set(checked);
    }
</script>

<Checkbox class="select-none" checked={$maxLevelStore} on:change={onChange}>Max level</Checkbox>
