use tauri::Manager

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn simple_command() {
    println!("I was invodked from JS!!")
}

#[tauri::command]
fn command_with_message(message: String) -> String {
    format!("hello {}", message)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let id = app.listen_global("front-to-back", |event| {
                println!(
                    "got front to back with payload {:?}",
                    event.payload().unwrap()
                )
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            simple_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

