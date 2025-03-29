// src/lib.rs

use tauri::{Manager, AppHandle};

mod mysql_installer;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Add this struct to manage state
#[derive(Default)]
struct AppState {
    _placeholder: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(AppState::default());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            mysql_installer::is_mysql_installed,
            mysql_installer::install_mysql
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}