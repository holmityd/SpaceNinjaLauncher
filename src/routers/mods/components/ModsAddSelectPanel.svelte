<script>
    import { Checkbox } from "flowbite-svelte";
    import { maxLevelStore } from "../store/Mods.store";

    function onChange({ target }) {
        const { checked } = target;
        maxLevelStore.set(checked);
    }
</script>

<Checkbox checked={$maxLevelStore} on:change={onChange}>Max level</Checkbox>
