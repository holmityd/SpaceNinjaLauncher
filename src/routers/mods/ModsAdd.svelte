<script>
    import Icon from "@iconify/svelte";
    import modsData from "../../../data/mods.json";
    import ModsCatalog from "./components/ModsCatalog.svelte";
    import { Button, Checkbox } from "flowbite-svelte";
    import { navigate } from "svelte-routing";
    import { addMods } from "../../services/user.service";

    let mods = Object.values(modsData)
        .filter((item) => item.name !== "Unfused Artifact" && item.type !== "Mod Set Mod")
        .map((item) => {
            const newItem = {
                ItemType: item.uniqueName,
                ...item,
            };
            // @ts-ignore
            if (item.fusionLimit) {
                // @ts-ignore
                newItem.UpgradeFingerprint = `{"lvl":${item.fusionLimit}}`;
            } else {
                // @ts-ignore
                newItem.ItemCount = 1;
            }
            return newItem;
        });

    // select logic
    let selected = [];
    let displayedList = [];
    function displayedListChange({ detail }) {
        displayedList = detail;
    }
    function selectMods(items) {
        let notExistingItems = items.filter(
            (item) => !selected.find((select) => select.ItemType === item.ItemType),
        );
        selected = [...selected, ...notExistingItems];
        notExistingItems.forEach((item) => {
            item.active = true;
        });
        mods = [...mods];
    }
    function unSelectMods(items) {
        selected = selected.filter(
            (select) => !items.find((item) => select.ItemType === item.ItemType),
        );
        items.forEach((item) => {
            item.active = false;
        });
        mods = [...mods];
    }

    // buttons
    function cardClick({ detail }) {
        if (selected.find((item) => item.ItemType === detail.ItemType)) {
            unSelectMods([detail]);
        } else {
            selectMods([detail]);
        }
    }
    function selectAllDisplayed() {
        selectMods(displayedList);
    }
    function clear() {
        unSelectMods(selected);
    }
    let maxLevel = true;
    function add() {
        const postItems = selected.map(({ ItemType, ItemCount, UpgradeFingerprint }) => ({
            ItemType,
            ItemCount: maxLevel ? ItemCount : ItemCount || 1,
            ...(maxLevel && { UpgradeFingerprint }),
        }));

        unSelectMods(selected);

        addMods(postItems);

        navigate("/dashboard/mods");
    }
</script>

<div class="container mx-auto flex flex-col">
    <ModsCatalog {mods} on:cardClick={cardClick} on:displayedListChange={displayedListChange} />
    <div class="box-b container fixed bottom-0 flex flex-row items-center gap-4 bg-gray-800 p-4">
        <p class="flex-grow">Selected {selected.length} mods</p>
        <Checkbox bind:checked={maxLevel}>Max level</Checkbox>
        <Button
            class="gap-1 border border-transparent hover:border-blue-600"
            on:click={selectAllDisplayed}
        >
            <Icon icon="iconoir:list-select" /> Select all
        </Button>
        <Button class="gap-1 border border-transparent hover:border-blue-600" on:click={clear}
            ><Icon icon="mdi:broom" /> Clear</Button
        >
        <Button
            class="gap-1 border border-transparent hover:border-green-600"
            color="green"
            on:click={add}><Icon icon="mdi:add" /> Add</Button
        >
    </div>
</div>
