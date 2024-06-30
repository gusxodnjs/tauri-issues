// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, Url, WebviewUrl, WebviewWindowBuilder};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn child(app_handle: AppHandle, label: &str, url: &str) {
    let parent_window = app_handle.get_webview_window("main").unwrap();
    let handle = app_handle.clone();
    let url = Url::parse(url).unwrap();
    let label = label.to_owned().clone();

    std::thread::spawn(move || {
        WebviewWindowBuilder::new(&handle, &label, WebviewUrl::External(url).into())
        .inner_size(500., 400.)
        .parent(&parent_window).unwrap()
        .decorations(false)
        .build()
        .unwrap();
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![child])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
