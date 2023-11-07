<script>
    import Icon from "@iconify/svelte";
    import { Button } from "flowbite-svelte";
    import { navigate } from "svelte-routing";

    function add() {
        navigate(window.location.pathname + "/add");
    }

    export let removeMode = false;
    function removeToggle() {
        removeMode = !removeMode;
    }
</script>

<div class="mt-4 flex justify-end gap-3">
    <Button
        class="gap-1 border border-transparent hover:border-green-600"
        color="green"
        on:click={add}><Icon icon="mdi:add" /> Add</Button
    >
    <Button
        class="gap-1 border border-transparent hover:border-red-600"
        color="red"
        on:click={removeToggle}
    >
        <Icon icon="mdi:delete" /> Remove mode
    </Button>
</div>
