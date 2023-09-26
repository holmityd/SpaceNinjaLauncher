<script>
    import { Button, Modal, Label, Input } from "flowbite-svelte";
    import ModCard from "./ModCard.svelte";
    import { updateMod } from "../../../services/user.service";
    import Icon from "@iconify/svelte";
    export let openModal = false;
    export let mod;

    $: editMod = { ...mod };

    $: lvl = editMod.UpgradeFingerprint ? JSON.parse(editMod.UpgradeFingerprint).lvl || 0 : 0;

    function onLevelInputChange(e) {
        const lvl = Number(e.target.value);
        editMod.UpgradeFingerprint = lvl > 0 ? JSON.stringify({ lvl }) : undefined;
    }

    function save() {
        const { ItemId, UpgradeFingerprint, ItemType } = editMod;
        updateMod({
            ItemId,
            UpgradeFingerprint,
            ItemType,
        });
    }
</script>

<Modal size="xs" title={editMod.name} bind:open={openModal} autoclose outsideclose>
    <div class="flex gap-4">
        <ModCard class="w-56" mod={editMod} position={{}} />
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
