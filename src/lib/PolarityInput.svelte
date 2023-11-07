<script>
    import PolarityInputSlot from "./PolarityInputSlot.svelte";
    /**
     * type {Array<{Slot: number; Value: string}>}
     */
    export let value = [];
    /**
     * @type {"suit"|"weapon"}
     */
    export let type;

    function getSlotsFromValue(items) {
        const newMap = new Map();
        items?.forEach((item) => newMap.set(item.Slot, item.Value));
        return newMap;
    }

    function onChange({ detail }) {
        if (!value) value = [];
        let slotIndex = value.findIndex((item) => item.Slot === detail.Slot);
        if (slotIndex != -1) {
            value[slotIndex] = detail;
        } else {
            value.push(detail);
        }
    }

    const polarities = {
        AP_TACTIC: {
            name: "Naramon",
            img: "https://static.wikia.nocookie.net/warframe/images/6/60/Naramon_Pol.svg",
        },
        AP_ATTACK: {
            name: "Madurai",
            img: "https://static.wikia.nocookie.net/warframe/images/b/b2/Madurai_Pol.svg",
        },
        AP_DEFENSE: {
            name: "Vazarin",
            img: "https://static.wikia.nocookie.net/warframe/images/6/6f/Vazarin_Pol.svg",
        },
        AP_POWER: {
            name: "Zenurik",
            img: "https://static.wikia.nocookie.net/warframe/images/8/8c/Zenurik_Pol.svg",
        },
        AP_UMBRA: {
            name: "Umbra",
            img: "https://static.wikia.nocookie.net/warframe/images/d/d2/Umbra_Pol.svg", // block [8,9]
            block: [8, 9],
        },
        AP_ANY: {
            name: "Aura",
            img: "https://static.wikia.nocookie.net/warframe/images/1/1b/Aura_Pol.svg", // only [8]
            block: [0, 1, 2, 3, 4, 5, 6, 7, 9],
        },
        AP_UNIVERSAL: {
            name: "",
            img: "",
            block: [8],
        },
    };

    let slotsByIndex = getSlotsFromValue(value);
</script>

<div class="relative">
    <p>Polarities</p>
    {#if type === "suit"}
        <div class="mt-3 grid grid-cols-4 gap-3">
            <span />
            <PolarityInputSlot
                index={8}
                polarity={slotsByIndex.get(8)}
                on:change={onChange}
                {polarities}
            />
            <PolarityInputSlot
                index={9}
                polarity={slotsByIndex.get(9)}
                on:change={onChange}
                {polarities}
            />
            <span />
        </div>
    {/if}
    <div class="mt-3 grid grid-cols-4 gap-3">
        {#each Array.from({ length: 8 }, (_, i) => 7 - i) as item}
            <PolarityInputSlot
                index={item}
                polarity={slotsByIndex.get(item)}
                on:change={onChange}
                {polarities}
            />
        {/each}
    </div>
</div>
