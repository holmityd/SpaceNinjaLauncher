<script>
    import { Command } from "@tauri-apps/api/shell";

    let output;

    async function launch() {
        const command = Command.sidecar("binaries/Warframe.x64", [
            "-fullscreen:0",
            "-graphicsDriver:dx11",
            "-gpuPreference:2",
            "-cluster:public",
            "-language:en",
            "-deferred:0",
        ]);
        output = await command.execute();
    }
</script>

<div
    class="w-1/3 flex-shrink-0 rounded-lg bg-gradient-to-br from-purple-600 to-blue-500 p-0.5 text-white hover:bg-gradient-to-bl focus:ring-blue-300 dark:focus:ring-blue-800"
>
    <button
        type="button"
        class="inline-flex w-full items-center justify-center rounded !border-0 bg-white px-5 py-2.5 text-center text-sm font-medium tracking-wider !text-gray-900 transition-all duration-75 ease-in hover:!text-inherit focus:outline-none focus:ring-4 group-hover:!bg-opacity-0 group-hover:!text-inherit dark:bg-gray-900 dark:!text-white"
        on:click={launch}>PLAY</button
    >
</div>
