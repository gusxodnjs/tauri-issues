// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default().on_window_event(|window, event| match event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
          window.hide().unwrap();
          api.prevent_close();
        }
        _ => {}
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
