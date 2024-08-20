use tauri::{ AppHandle, Manager, Wry };
use tauri::menu::{ Menu, MenuBuilder, PredefinedMenuItem, SubmenuBuilder };

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn create_menus(app_handle: &AppHandle) -> Menu<Wry> {
    let menu = MenuBuilder::new(app_handle).build().unwrap();
    let undo = PredefinedMenuItem::undo(app_handle, Some("undo")).unwrap();
    let redo = PredefinedMenuItem::redo(app_handle, Some("redo")).unwrap();

    let submenu = SubmenuBuilder::new(app_handle, "edit").build().unwrap();
    submenu.append(&undo).unwrap();
    submenu.append(&redo).unwrap();

    menu.append(&submenu).unwrap();
    return menu;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {
            let menu = create_menus(app.app_handle());
            app.set_menu(menu).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
