// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use kdbx_rs::{self, binary::Unlocked, CompositeKey, Database as KdbxDatabase, Error};
use models::kdbx_keys;
use serde::{Deserialize, Serialize, Serializer};
use std::sync::Mutex;

use crate::models::kdbx_keys::*;
use crate::models::key::*;
use tauri::{App, Manager, State, Window};

pub mod config;
pub mod models;
mod repository;

// #[derive(Serialize, Deserialize)]
struct InternalState {
    database_path: Mutex<String>,
    database: Mutex<kdbx_rs::database::Database>,
}

impl Default for InternalState {
    fn default() -> Self {
        Self {
            database_path: Mutex::new("".to_string()),
            database: Mutex::new(kdbx_rs::database::Database::default()),
        }
    }
}

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
                window.hide().unwrap();
                println!("Initializing application");
                build_application();
                // std::thread::sleep(std::time::Duration::from_secs(5));
                println!("Done initializing.");

                window.show().unwrap();
                splashscreen_window.close().unwrap();
                window.set_title("Key Manager").unwrap();
            });

            Ok(())
        })
        .manage(InternalState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_keys,
            create_new_key,
            upload_kdbx_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_keys(state: State<InternalState>) -> kdbx_keys::Database {
    // let keys = Key::retrive_keys_from_db().unwrap();

    // keys
    // *state
    // .database
    // .lock()
    // .unwrap()
    //     .entries()
    //     .iter()
    //     .map(|entry| {
    //         Key::new(
    //             Some(entry.uuid().to_string()),
    //             entry.title().to_string(),
    //             &config::Config::default().password_options,
    //         )
    //     })
    //     .collect()

    println!("called");
    let db = kdbx_keys::Database::new(state.database.lock().unwrap().clone());
    println!("db: {:#?}", db);
    db
}

#[tauri::command]
fn create_new_key(window: Window, name: String) -> () {
    let key = Key::new(None, name, &config::Config::default().password_options);
    key.persist().unwrap();
    window.emit("key_created", key).unwrap();
}

#[tauri::command]
fn upload_kdbx_database(
    // window: Window,
    state: State<InternalState>,
    // path: BufReader<R>,
    path: String,
    password: String,
) -> Result<(), String> {
    // println!("Database path: {}", path);
    //TODO: Check if file exists

    let mut database_path = state.database_path.lock().unwrap();
    *database_path = path;
    println!("Database path: {}", database_path);
    println!("Database password: {password}");

    let kdbx = kdbx_rs::open(&database_path.to_string()).unwrap();
    let key = CompositeKey::from_password(&password);
    let mut unlocked = kdbx
        .unlock(&key)
        .map_err(|_| println!("Error opening the database"))
        .unwrap();
    // .unwrap();
    let mut database_unlocked = state.database.lock().unwrap();

    //TODO: add a mutable reference to the database to the state. I am really not sure that cloning
    //is the best way to do this.
    *database_unlocked = unlocked.database_mut().clone();
    // println!("Database unlocked: {:#?}", database_unlocked);
    Ok(())

    // window.emit("key_created", key).unwrap();
}
