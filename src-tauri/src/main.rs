// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn run_applescript(script: &str) {
    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to run AppleScript");
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![run_applescript])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
