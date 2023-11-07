
# SpaceNinjaLauncher

Simple launcher for the project [SpaceNinjaServer](https://github.com/AngeloTadeucci/SpaceNinjaServer). It contains admin tools and the server itself. The launcher file also auto-launches the server file.

## Installation
You can choose to either download the .exe files or generate them by yourself.

### Download
1. Download latest [release](https://github.com/holmityd/SpaceNinjaLauncher/releases) version
2. Paste the downloaded files into your game directory.
3. Run "WarframeLauncher.exe"

### Generate 
##### Prerequisites
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Install MongoDB](https://www.mongodb.com/)

##### Getting the Build Folder
1. ```git clone git@github.com:holmityd/SpaceNinjaServer.git```
2. ```cd SpaceNinjaServer```
3. ```git checkout bundled-build-2```
4. ```npm install```
5. Follow the steps in the "@install guide" (Set up the .env and .bin files).
6. ```npm run build```

##### Install
1. Copy and paste the build folder to the root of your project.
2. ```npm install```


## Build
1. ```npm run tauri:build```
2. Copy "WarframeLauncher.exe" and "WarframeServer.exe" from "src-tauri/target/release" and place them in the folder with the game.

## Dev
1. ```npm run dev``` - Run the frontend in the Svelte framework
2. ```npm run tauri:dev``` - Run both the frontend and backend
3. ```npm run package``` - Create binary files (build folder -> .exe) (If you encounter a build error related to the node version, consider downgrading to node version 18)
4. ```npm run update-items``` - Update items from the warframe-items package

## Recommended IDE Setup
[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
