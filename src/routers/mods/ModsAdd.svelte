<script>
    import modsData from "../../../data/mods.json";
    import Button from "../../components/Button.svelte";
    import ModsCatalog from "./components/ModsCatalog.svelte";

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
    function add() {
        console.log(
            "add",
            selected.map(({ ItemType, ItemCount, UpgradeFingerprint }) => ({
                ItemType,
                ...(ItemCount !== undefined && { ItemCount }),
                ...(UpgradeFingerprint !== undefined && { UpgradeFingerprint }),
            })),
        );
    }
</script>

<ModsCatalog {mods} on:cardClick={cardClick} on:displayedListChange={displayedListChange} />
<div class="box-b container fixed bottom-0 flex-row items-center gap-4 bg-gray-800 p-4">
    <p class="flex-grow">Selected {selected.length} mods</p>
    <Button on:click={selectAllDisplayed}>Select all</Button>
    <Button on:click={clear}>Clear</Button>
    <Button on:click={add}>Add</Button>
</div>
