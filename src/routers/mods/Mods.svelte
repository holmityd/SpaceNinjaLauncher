<script>
    import modsData from "../../../data/mods.json";
    import ModsCatalog from "./components/ModsCatalog.svelte";
    import Icon from "@iconify/svelte";
    import { userStore } from "../../store/User.store";
    import { navigate } from "svelte-routing";
    import { Button } from "flowbite-svelte";
    import { removeMods } from "../../services/user.service";
    import { onMount } from "svelte";
    import ModModalInfo from "./components/ModModalInfo.svelte";

    let mods;

    // select logic
    let selected = [];
    let displayedList = [];
    function displayedListChange({ detail }) {
        displayedList = detail;
    }
    function selectMods(items) {
        let notExistingItems = items.filter(
            (item) =>
                !selected.find(
                    (select) =>
                        select.ItemType === item.ItemType &&
                        select.UpgradeFingerprint === item.UpgradeFingerprint,
                ),
        );
        selected = [...selected, ...notExistingItems];
        notExistingItems.forEach((item) => {
            item.active = true;
        });
        mods = [...mods];
    }
    function unSelectMods(items) {
        selected = selected.filter(
            (select) =>
                !items.find(
                    (item) =>
                        select.ItemType === item.ItemType &&
                        select.UpgradeFingerprint === item.UpgradeFingerprint,
                ),
        );
        items.forEach((item) => {
            item.active = false;
        });
        mods = [...mods];
    }
    function modClickToSelect(detail) {
        if (
            selected.find(
                (item) =>
                    item.ItemType === detail.ItemType &&
                    item.UpgradeFingerprint === detail.UpgradeFingerprint,
            )
        ) {
            unSelectMods([detail]);
        } else {
            selectMods([detail]);
        }
    }

    // removeMode
    let removeMode = false;
    function removeToggle() {
        removeMode = !removeMode;
        unSelectMods(selected);
    }

    // buttons
    function cardClick({ detail }) {
        if (!removeMode) {
            modModalOpen = true;
            modModalInfo = detail;
        } else {
            modClickToSelect(detail);
        }
    }
    function selectAllDisplayed() {
        selectMods(displayedList);
    }
    function clear() {
        unSelectMods(selected);
    }
    function remove() {
        const postItems = selected.map(({ ItemType, ItemCount, UpgradeFingerprint, _id }) => ({
            ItemType,
            ItemCount,
            UpgradeFingerprint,
            _id,
        }));

        unSelectMods(selected);

        removeMods(postItems);

        removeToggle();
    }
    function add() {
        navigate("/dashboard/mods/add");
    }

    let modModalOpen = false;
    let modModalInfo;

    onMount(() => {
        const stopModSubsciption = userStore.subscribe((value) => {
            mods = [...value.inventory.RawUpgrades, ...value.inventory.Upgrades]
                .map(({ ItemType, ItemCount, UpgradeFingerprint, _id }) => ({
                    ...modsData[ItemType],
                    ItemType,
                    ItemCount,
                    UpgradeFingerprint,
                    _id,
                }))
                .filter((i) => !!i.uniqueName);
        });
        return () => {
            stopModSubsciption();
        };
    });
</script>

<div class="container mx-auto box-border flex flex-col">
    <div class="mt-4 flex justify-end gap-4">
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

    {#if mods}
        <ModsCatalog {mods} on:cardClick={cardClick} on:displayedListChange={displayedListChange} />
    {/if}

    {#if removeMode}
        <div
            class="box-b container fixed bottom-0 flex flex-row items-center gap-4 bg-gray-800 p-4"
        >
            <p class="flex-grow">Selected {selected.length} mods</p>
            <Button
                class="gap-1 border border-transparent hover:border-blue-600"
                on:click={selectAllDisplayed}
            >
                <Icon icon="iconoir:list-select" /> Select all
            </Button>
            <Button class="gap-1 border border-transparent hover:border-blue-600" on:click={clear}>
                <Icon icon="mdi:broom" /> Clear
            </Button>
            <Button
                class="gap-1 border border-transparent hover:border-red-600"
                color="red"
                on:click={remove}><Icon icon="mdi:delete" /> Remove</Button
            >
        </div>
    {/if}
</div>

<ModModalInfo bind:openModal={modModalOpen} mod={modModalInfo} />
