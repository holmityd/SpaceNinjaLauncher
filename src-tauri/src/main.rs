// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{api::process::Command, Manager};

mod server;
use std::thread;

fn main() {
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
