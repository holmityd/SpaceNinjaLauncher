<script>
    import Input from "../components/Input.svelte";
    export let value = "";
    export let id = undefined;
</script>

<Input {id} bind:value placeholder="Search..." />
