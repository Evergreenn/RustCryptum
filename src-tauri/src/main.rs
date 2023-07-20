// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::models::key::*;
use tauri::Manager;

pub mod config;
pub mod models;
mod repository;

fn build_application() {
    repository::init();
    config::Config::load_config(&config::Config::default());
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let window = app.get_window("main").unwrap();

            tauri::async_runtime::spawn(async move {
                // window.close().unwrap();
                window.hide().unwrap();
                println!("Initializing application");
                build_application();
                std::thread::sleep(std::time::Duration::from_secs(5));
                println!("Done initializing.");

                window.show().unwrap();
                splashscreen_window.close().unwrap();
                window.set_title("Key Manager").unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_keys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_keys() -> Vec<Key> {
    let keys = Key::retrive_keys_from_db().unwrap();
    keys
}
