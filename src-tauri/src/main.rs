// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{api::process::Command, Manager};

mod server;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::{
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    // Initialize Discord Rich Presence
    let mut client = DiscordIpcClient::new("1180118975934382090").unwrap();
    client.connect().unwrap();

    let version: &str = &format!("v{}", env!("CARGO_PKG_VERSION"));
    let name = env!("CARGO_PKG_DESCRIPTION");
    let start_time = SystemTime::now();
    let unix_time = start_time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

    client
        .set_activity(
            activity::Activity::new()
                .state(version)
                .assets(activity::Assets::new().large_image("big").large_text(name))
                .timestamps(activity::Timestamps::new().start(unix_time)),
        )
        .unwrap();

    // Initialize Tauri
    tauri::Builder::default()
        .setup(|app| {
            // sidecar for node server
            app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                Command::new_sidecar("WarframeServer")
                    .expect("failed to setup `app` sidecar")
                    .spawn()
                    .expect("Failed to spawn packaged node");
            });

            // seperate thread for actix-web
            let handle = app.handle();
            let boxed_handle = Box::new(handle);
            thread::spawn(move || {
                server::init(*boxed_handle).unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
