<script>
    import pages from "../routers";
    import Icon from "@iconify/svelte";
    import { navigate } from "svelte-routing";
    import UserPanel from "./UserPanel.svelte";
    import { onDestroy, onMount } from "svelte";
    import { modalPage } from "../store/User.store";
    import { get } from "svelte/store";

    const page = pages.find((i) => i.path === window.location.pathname);

    function back() {
        navigate(page.back);
    }

    function backListener(event) {
        if (event.key === "Escape" && !get(modalPage)) {
            back();
        }
    }
    onMount(() => {
        document.addEventListener("keydown", backListener);
    });
    onDestroy(() => {
        document.removeEventListener("keydown", backListener);
    });
</script>

<div class="bg-blue-900">
    <div class="container mx-auto flex flex-row items-center justify-between px-4 py-1.5">
        {#if page.back}
            <button
                on:click={back}
                class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-transparent p-0 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-4 focus:ring-blue-800"
            >
                <Icon icon="fluent-mdl2:back" />
            </button>
        {:else}
            <span />
        {/if}
        <h3 class="text-xl">{page.title}</h3>
        <UserPanel />
    </div>
</div>
