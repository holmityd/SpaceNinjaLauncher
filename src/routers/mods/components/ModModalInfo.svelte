<script>
    import { Button, Modal, Label, Input } from "flowbite-svelte";
    import ModCard from "./ModCard.svelte";
    import { updateMod } from "../../../services/user.service";
    import Icon from "@iconify/svelte";
    import { onMount } from "svelte";
    import { modalPage } from "../../../store/User.store";
    export let openModal = false;
    export let item;

    $: editMod = { ...item };

    $: lvl = editMod.UpgradeFingerprint ? JSON.parse(editMod.UpgradeFingerprint).lvl || 0 : 0;

    function onLevelInputChange(e) {
        const lvl = Number(e.target.value);
        editMod.UpgradeFingerprint = lvl > 0 ? JSON.stringify({ lvl }) : undefined;
    }

    function save() {
        const { _id, UpgradeFingerprint, ItemType } = editMod;
        updateMod({
            _id,
            UpgradeFingerprint,
            ItemType,
        });
        openModal = false;
    }

    
    onMount(() => {
        modalPage.set(true);
        return () => {
            setTimeout(()=>{
                modalPage.set(false);
            });
        };
    });
</script>

<Modal size="xs" title={editMod.name} bind:open={openModal} outsideclose>
    <div class="flex gap-4">
        <ModCard class="w-56" item={editMod} position={{}} />
        <div class="mb-6">
            <Label for="mod-fusion-level" class="mb-2 block">Fusion level</Label>
            <Input
                max={editMod.fusionLimit}
                min={0}
                step={1}
                type="number"
                value={lvl}
                on:change={onLevelInputChange}
                id="mod-fusion-level"
            />
        </div>
    </div>

    <svelte:fragment slot="footer">
        <div class="flex w-full justify-end">
            <Button class="gap-1 border border-transparent hover:border-blue-600" on:click={save}>
                <Icon icon="prime:save" /> Save
            </Button>
        </div>
    </svelte:fragment>
</Modal>
