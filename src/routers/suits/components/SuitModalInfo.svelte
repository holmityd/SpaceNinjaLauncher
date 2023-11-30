<script>
    import { Button, Modal, Label, Input } from "flowbite-svelte";
    import SuitCard from "./SuitCard.svelte";
    import Icon from "@iconify/svelte";
    import PolarityInput from "../../../lib/PolarityInput.svelte";
    import { updateSuit } from "../../../services/user.service";
    import { getSuitLvlByXP, suitLvlXP } from "../services/suit.service";
    export let openModal = false;
    export let item;

    $: editSuit = { ...item };

    let lvl = getSuitLvlByXP(item.XP);

    function changeLvl(e) {
        let lvl = parseInt(e.target.value) - 1;
        if (lvl < 0) {
            editSuit.XP = 0;
        } else {
            editSuit.XP = suitLvlXP[lvl];
        }
    }

    function changeExp(e) {
        editSuit.XP = parseInt(e.target.value);
        lvl = getSuitLvlByXP(editSuit.XP);
    }

    function changePolarized(e) {
        editSuit.Polarized = parseInt(e.target.value);
    }

    $: polarity = item.Polarity;

    function save() {
        const {
            ItemType,
            Configs,
            XP,
            InfestationDate,
            UpgradeVer,
            Features,
            Polarized,
            ModSlotPurchases,
            _id,
        } = editSuit;

        updateSuit({
            ItemType,
            Configs,
            XP,
            InfestationDate,
            UpgradeVer,
            Features,
            Polarity: polarity,
            Polarized,
            ModSlotPurchases,
            _id,
        });
        openModal = false;
    }
</script>

<Modal size="xs" title={editSuit.info.name} bind:open={openModal} outsideclose>
    <div class="flex gap-4">
        <SuitCard item={editSuit} />
        <div>
            <Label for="suit-lvl" class="mb-2 block">Level</Label>
            <Input
                id="suit-lvl"
                value={lvl}
                max="30"
                min="0"
                type={"number"}
                on:change={changeLvl}
            />

            <Label for="suit-xp" class="mb-2 mt-4 block">XP</Label>
            <Input
                step="1000"
                id="suit-xp"
                value={editSuit.XP}
                min="0"
                type={"number"}
                on:change={changeExp}
            />

            <Label for="suit-polarized" class="mb-2 mt-4 block">Polarized</Label>
            <Input
                id="suit-polarized"
                value={editSuit.Polarized}
                min="0"
                type={"number"}
                on:change={changePolarized}
            />
        </div>
    </div>
    <PolarityInput type="suit" bind:value={polarity} />

    <svelte:fragment slot="footer">
        <div class="flex w-full justify-end">
            <Button class="gap-1 border border-transparent hover:border-blue-600" on:click={save}>
                <Icon icon="prime:save" /> Save
            </Button>
        </div>
    </svelte:fragment>
</Modal>
