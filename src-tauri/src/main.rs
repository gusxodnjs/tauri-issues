// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::AppHandle;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn hide(app_handle: AppHandle) {
    let tray_icon = app_handle.tray_by_id("tray");
    if tray_icon.is_some() {
        tray_icon.unwrap().set_visible(false).unwrap();
    }
}

#[tauri::command]
fn show(app_handle: AppHandle) {
    let tray_icon = app_handle.tray_by_id("tray");
    if tray_icon.is_some() {
        tray_icon.unwrap().set_visible(true).unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![hide, show])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
