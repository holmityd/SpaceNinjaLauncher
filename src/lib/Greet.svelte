<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { Command } from "@tauri-apps/api/shell";

  let name = "";
  let greetMsg = "";

  async function launch() {
    const command = Command.sidecar("binaries/Warframe.x64", [
      "-fullscreen:0",
      "-graphicsDriver:dx11",
      "-gpuPreference:2",
      "-cluster:public",
      "-language:en",
      "-deferred:0",
    ]);
    const output = await command.execute();
  }

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
    launch();
    // await invoke("launch");
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>
