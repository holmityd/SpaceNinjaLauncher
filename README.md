
# SpaceNinjaLauncher

Simple launcher for the project [SpaceNinjaServer](https://github.com/AngeloTadeucci/SpaceNinjaServer). It contains admin tools and the server itself. The launcher file also auto-launches the server file.

## Installation
You can choose to either download the .exe files or generate them by yourself.

### Download
1. Download latest [release](https://github.com/holmityd/SpaceNinjaLauncher/releases) version
2. Paste the downloaded files into your game directory.
3. Run "SpaceNinjaLauncher.exe"

### Generate 
[Install Rust](https://www.rust-lang.org/tools/install)

1. ```git clone git@github.com:holmityd/SpaceNinjaLauncher.git```
2. ```cd SpaceNinjaLauncher```
3. ```git clone -b bundled-build-4 git@github.com:holmityd/SpaceNinjaServer.git```
4. ```cd SpaceNinjaServer```
5. ```npm install```
6. prepare .env and [cache data](https://discord.com/channels/1108159019635462206/1108165338048249866/1109158541060743259)
7. ```cd ..```
8. create build folder and place [mongod.exe](https://www.mongodb.com/try/download/community) inside - on downloading choose as zip package
9. ```npm install``` - packaging may fail cause node version, use node18 and rerun ```npm run package```
10. ```npm run tauri:build```
11. binary files in folder "src-tauri/target/release" (SpaceNinjaLauncher.exe, SpaceNinjaServer.exe), copy&place to game folder

Note: The default database for the launcher is "mongodb://127.0.0.1:27017/openWF". If you are using a different database, please add the "mongodbUrl" field to the configuration file as shown below:

```json
{
  "mongodbUrl": "mongodb://127.0.0.1:27017/wf_emulator",
  "logPath": "C:\\Users\\USERNAME\\Desktop\\WFLogs",
  "serverType": "local"
}
```

## npm scripts
1. ```dev``` - front
2. ```tauri:dev``` - front&back
3. ```update-items``` - Update items from the warframe-items package
4. ```build-server``` - creates bundled build (index.js and other files)
5. ```mongo-workaround``` - adds mongo-workaround.js script to build/index.js
6. ```package``` - Create binary files (build folder -> .exe) (If you encounter a build error related to the node version, consider downgrading to node version 18)

## Recommended IDE Setup
[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
