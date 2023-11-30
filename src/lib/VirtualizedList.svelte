<script>
    import { onDestroy, onMount } from "svelte";
    import Grid from "./Grid.svelte";

    export let itemComponent;

    export let list;

    // scroll optimization
    let gridElm;
    let gridParams;
    let gridItemsPosition = [];
    let startingIndex;
    let canShowCount;
    let gridHeight = 0;
    function gridInit() {
        updateGridParams();
        getAllGridItemsPosition();
        getVisibleItems();
    }
    function updateGridParams() {
        const gridRect = gridElm.getBoundingClientRect();
        const { width, height } = gridElm.firstElementChild.getBoundingClientRect();

        gridParams = {
            width,
            height,
            gridWidth: gridRect.width,
            rowItemsCount: 0,
            gap: 0,
            responsive: [],
        };

        gridParams.rowItemsCount = Math.floor(gridParams.gridWidth / width);
        gridParams.gap =
            (gridParams.gridWidth - gridParams.rowItemsCount * width) /
            (gridParams.rowItemsCount - 1);

        updateGridHeight();
    }
    function updateGridHeight() {
        if (!gridParams) return;
        const { rowItemsCount, height, gap } = gridParams;
        gridHeight = Math.ceil(list.length / rowItemsCount) * (height + gap) - gap;
    }
    function getVisibleItems() {
        const { height, rowItemsCount } = gridParams;
        const columnElements = Math.ceil(window.innerHeight / height);
        canShowCount = columnElements * rowItemsCount * 3; // 3 - prev, current, next screen
        startingIndex = gridItemsPosition.findIndex(
            (i) => i.top + i.height * columnElements >= window.scrollY,
        );
    }
    function getAllGridItemsPosition() {
        if (!gridParams) return;
        const { rowItemsCount, height, width, gap } = gridParams;
        for (let i = 0; i < list.length; i++) {
            const [columnIndex, rowIndex] = [Math.floor(i / rowItemsCount), i % rowItemsCount];

            gridItemsPosition[i] = {
                width,
                height,
                top: (height + gap) * columnIndex,
                left: (width + gap) * rowIndex,
            };
        }
    }

    $: {
        if (list) {
            updateGridHeight();
            getAllGridItemsPosition();
        }
    }

    // lifecycle
    onMount(async () => {
        gridInit();

        window.addEventListener("scroll", getVisibleItems);
        window.addEventListener("resize", gridInit);
    });
    onDestroy(() => {
        window.removeEventListener("scroll", getVisibleItems);
        window.removeEventListener("resize", gridInit);
    });
</script>

<Grid
    on:elRef={({ detail }) => {
        gridElm = detail;
    }}
    style={`height: ${gridHeight}px`}
>
    {#if itemComponent}
        <div class="invisible">
            <svelte:component this={itemComponent} />
        </div>

        {#each list as item, index}
            {#if startingIndex <= index && index < startingIndex + canShowCount}
                <div
                    style={gridItemsPosition[index]?.top !== undefined &&
                        `position: absolute;top: ${gridItemsPosition[index].top}px; left: ${gridItemsPosition[index].left}px; width: ${gridItemsPosition[index].width}px; height: ${gridItemsPosition[index].height}px`}
                >
                    <svelte:component this={itemComponent} {item} on:cardClick />
                </div>
            {/if}
        {/each}
    {/if}
</Grid>
