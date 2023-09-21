# Svelte + Vite

This template should help get you started developing with Tauri and Svelte in Vite.

## Installation

Follow these steps to set up the project:

1. **Prepare Packages**  
   Run the following command to install the necessary packages:

   ```
   npm install
   ```

2. **Paste Build Folder**  
    Copy the `build` folder from the server repository and paste it into the root directory of this project.

3. **Package the Application**  
    Execute the command below to package the application: (Note: need Node18 version)

    ```
    npm run package
    ```
    
4. **Create Empty File**  
    Create an empty file named `Warframe.x64<current platform>.exe` inside the `src-tauri/binaries/` directory. The `<current platform>` part can be copied from the already generated `WarframeServer` file located in the same `binaries` folder.

5. **Build the Tauri Application**  
    Run the following command to build the Tauri application. Ensure you copy the `WarframeLauncher` and `WarframeServer` files from `src-tauri/target/release` after this step. (Note: `Warframe.x64` is for the launch sidecar button and is not needed for this step.)

    ```
    npm run tauri-build
    ```


## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

