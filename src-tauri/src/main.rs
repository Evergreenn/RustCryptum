// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use kdbx_rs::{self, CompositeKey};
use kdbx_rs::database::Entry;
use models::kdbx_keys;
use passwords::PasswordGenerator;
use std::sync::{Arc, Mutex};

use crate::models::key::*;
use tauri::{Manager, State, Window};

pub mod config;
pub mod models;
mod repository;

// #[derive(Serialize, Deserialize)]
struct InternalState {
    database_path: Mutex<String>,
    database_password: Mutex<String>,
    // database: Arc<Mutex<&'a mut kdbx_rs::database::Database>>,
}

impl Default for InternalState{
    fn default() -> Self {
        Self {
            database_path: Mutex::new("".to_string()),
            database_password: Mutex::new("".to_string()),
            // database: Arc::new(Mutex::new(& mut kdbx_rs::database::Database::default())),
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
            upload_kdbx_database,
            generate_password
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
    // let db = kdbx_keys::Database::new(state.database.lock().unwrap().clone());
    let database_path = state.database_path.lock().unwrap(); 
    let database_password = state.database_password.lock().unwrap();

    let kdbx = kdbx_rs::open(&database_path.to_string()).unwrap();
    let key = CompositeKey::from_password(&database_password);
    let mut unlocked = kdbx.unlock(&key).map_err(|e| e.1.to_string()).unwrap();

    let db = kdbx_keys::Database::new(unlocked.database_mut());
    println!("db_groups: {:#?}", db.groups);
    db
}

#[tauri::command]
fn create_new_key(state: State<InternalState>,name: String, password: String, currentGroup: String, username: String, url: String) -> () {

    // let mut database = state.database.lock().unwrap();
    // let mut group = database.find_group_mut(|g|g.name() == &currentGroup).unwrap();
    //
    
    println!("name: {}", name);
    println!("password: {}", password);
    println!("currentGroup: {}", currentGroup);
    println!("username: {}", username);
    println!("url: {}", url);


    let database_path = state.database_path.lock().unwrap(); 
    let database_password = state.database_password.lock().unwrap();

    let kdbx = kdbx_rs::open(&database_path.to_string()).unwrap();
    let key = CompositeKey::from_password(&database_password);
    let mut unlocked = kdbx.unlock(&key).map_err(|e| e.1.to_string()).unwrap();
    let mut database = unlocked.database_mut();

    let mut group = database.find_group_mut(|g|g.name() == &currentGroup).unwrap();
    println!("group: {:#?}", group);
    let mut entry = Entry::default();
    entry.set_title(&name);
    entry.set_username(&username);
    entry.set_password(&password);
    entry.set_url(&url);

    println!("entry: {:#?}", entry);
    group.add_entry(entry);


    // println!("group: {:#?}", group);
    
    


    // let key = Key::new(None, name, &config::Config::default().password_options);
    // key.persist().unwrap();
    // window.emit("key_created", key).unwrap();
}

#[tauri::command]
// fn generate_password(window: Window, options: config::PasswordOptions) -> String {
fn generate_password(window: Window, options: config::PasswordOptions) -> String {
    let pg = PasswordGenerator::new()
        .length(options.length as usize)
        .numbers(options.use_numbers)
        .lowercase_letters(options.use_lowercase)
        .uppercase_letters(options.use_uppercase)
        .symbols(options.use_symbols)
        .spaces(options.use_spaces)
        .exclude_similar_characters(options.exclude_similar_characters)
        .strict(true);
    let password = pg.generate_one().unwrap();
    password
    // let password = Key::generate_password(&password_options);
    // window.emit("password_generated", password.clone()).unwrap();
    // Ok(password)
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

    let mut database_password = state.database_password.lock().unwrap();
    *database_password = password;


    // let kdbx = kdbx_rs::open(&database_path.to_string()).unwrap();
    // let key = CompositeKey::from_password(&password);
    // let mut unlocked = kdbx.unlock(&key).map_err(|e| e.1.to_string())?;
    // println!("Database unlocked: {:#?}", unlocked.database());
    // let mut database_unlocked = state.database.lock().unwrap();
    // let refto = Arc::new(Mutex::new(unlocked.database_mut()));
    

    //TODO: add a mutable reference to the database to the state. I am really not sure that cloning
    //is the best way to do this.
    // *database_unlocked = unlocked.database_mut();
    // println!("Database unlocked: {:#?}", database_unlocked);
    Ok(())

    // window.emit("key_created", key).unwrap();
}

// fn manage_database(database_path: String, database_password: String) -> Result<&'static mut kdbx_rs::database::Database, String> {
    // let kdbx = kdbx_rs::open(&database_path.to_string()).unwrap();
    // let key = CompositeKey::from_password(&database_password);
    // let mut unlocked = kdbx.unlock(&key).map_err(|e| e.1.to_string())?;

    // Ok(unlocked.database_mut())
// } 
