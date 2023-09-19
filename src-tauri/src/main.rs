#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod bindings;
use bindings::themes::query_settings_theme;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![query_settings_theme])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
