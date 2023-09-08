// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use kdbx_rs::database::{Database, Entry, Group};
use kdbx_rs::{self, CompositeKey, Kdbx};
use models::kdbx_keys;
use passwords::PasswordGenerator;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::{thread, time};

// use crate::models::key::*;
use tauri::{Manager, State, Window};

pub mod config;
pub mod models;
mod repository;
// pub mod utils;

// #[derive(Serialize, Deserialize)]
struct InternalState {
    database_path: Mutex<String>,
    database_password: Mutex<String>,
    database: Arc<Mutex<kdbx_rs::database::Database>>,
    config: Mutex<config::Config>,
}

impl Default for InternalState {
    fn default() -> Self {
        Self {
            database_path: Mutex::new("".to_string()),
            database_password: Mutex::new("".to_string()),
            database: Arc::new(Mutex::new(kdbx_rs::database::Database::default())),
            config: Mutex::new(config::Config::default()),
        }
    }
}

fn build_application() {
    // todo!("Build application")
    // repository::init();
    // TODO: Load config from file
    // config::Config::load_config(&config::Config::default());
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
                println!("Done initializing.");

                window.show().unwrap();
                splashscreen_window.close().unwrap();
                window.set_title("Key Manager").unwrap();
            });

            Ok(())
        })
        .manage(InternalState::default())
        .invoke_handler(tauri::generate_handler![
            get_keys,
            create_new_key,
            create_new_folder,
            upload_kdbx_database,
            generate_password,
            shutdown,
            lock,
            get_one_key,
            get_config,
            set_theme,
            create_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_config(state: State<InternalState>) -> config::Config {
    let mut config = state.config.lock().unwrap();
    *config = config::Config::load_config(&config::Config::default());
    config.load_config()
}

#[tauri::command]
fn set_theme(theme: String, state: State<InternalState>) -> () {
    let mut config = state.config.lock().unwrap();
    config.set_color_scheme(theme);

    // *config = config::Config::load_config(&config::Config::default());
    // config.load_config()
}

#[tauri::command]
fn get_keys(state: State<InternalState>) -> kdbx_keys::Database {
    let db = kdbx_keys::Database::new(state.database.lock().unwrap().clone());
    // println!("db_groups: {:#?}", db.groups);
    db
}

#[tauri::command]
fn get_one_key(state: State<InternalState>, id: String) -> kdbx_keys::Entry {
    // let db = kdbx_keys::Database::new(state.database.lock().unwrap().clone());
    // let entry = db.get_entry(id);
    let mut database = state.database.lock().unwrap();
    let entry = database.find_entry(|e| e.uuid().to_string() == id).unwrap();
    let entry = kdbx_keys::Entry::new(entry.clone());
    entry
}

#[tauri::command(async)]
fn create_new_folder(
    window: Window,
    state: State<InternalState>,
    name: String,
    current_group_uuid: String,
) -> () {
    let mut database = state.database.lock().unwrap();

    let database_path = state.database_path.lock().unwrap();
    let database_password = state.database_password.lock().unwrap();

    let group = database.find_group_mut(|g| g.uuid().to_string() == current_group_uuid);

    let group = if group.is_none() {
        database.root_mut()
    } else {
        group.unwrap()
    };

    let mut new_group = Group::default();
    new_group.set_name(&name);
    group.add_group(new_group);

    let mut kdbx = Kdbx::from_database(database.clone());
    kdbx.set_key(CompositeKey::from_password(&database_password.clone()))
        .unwrap();
    let file = File::create(database_path.clone());
    match file {
        Ok(mut file) => {
            kdbx.write(&mut file).unwrap();
        }
        Err(e) => panic!("Error creating file: {}", e),
    };

    window.emit("refresh_ui", {}).unwrap();
}

#[tauri::command(async)]
fn create_new_key(
    window: Window,
    state: State<InternalState>,
    name: String,
    password: String,
    current_group_uuid: String,
    username: String,
    url: String,
) -> () {
    let mut database = state.database.lock().unwrap();
    let group = database.find_group_mut(|g| g.uuid().to_string() == current_group_uuid);

    let group = if group.is_none() {
        database.root_mut()
    } else {
        group.unwrap()
    };

    let mut entry = Entry::default();
    entry.set_title(&name);
    entry.set_username(&username);
    entry.set_password(&password);
    entry.set_url(&url);

    group.add_entry(entry);

    save_database(database.clone(), &state);

    window.emit("refresh_ui", {}).unwrap();
}

fn save_database(database: kdbx_rs::database::Database, state: &State<InternalState>) {
    let database_path = state.database_path.lock().unwrap();
    let database_password = state.database_password.lock().unwrap();

    let mut kdbx = Kdbx::from_database(database);
    kdbx.set_key(CompositeKey::from_password(&database_password.clone()))
        .unwrap();
    let file = File::create(database_path.clone());
    match file {
        Ok(mut file) => {
            kdbx.write(&mut file).unwrap();
        }
        Err(e) => panic!("Error creating file: {}", e),
    };
}

#[tauri::command]
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
}

#[tauri::command(async)]
fn create_database(
    window: Window,
    state: State<InternalState>,
    path: String,
    password: String,
    name: String,
    description: String,
) -> Result<(), String> {
    let mut database_password = state.database_password.lock().unwrap();
    *database_password = password;

    let mut database = state.database.lock().unwrap();
    let mut new_database = Database::default();
    new_database.set_name(&name);
    new_database.set_description(&description);
    let mut kdbx = Kdbx::from_database(new_database.clone());
    *database = new_database;
    // new_database.set_root(Group::default());

    // kdbx.root_mut().set_name("Root");
    kdbx.set_key(CompositeKey::from_password(&database_password.clone()))
        .unwrap();

    let full_path = format!("{}/{}.{}", path, &name, "kdbx");
    // println!("full_path: {}", full_path);

    let mut database_path = state.database_path.lock().unwrap();
    *database_path = full_path.clone();

    let file = File::create(&full_path);
    match file {
        Ok(mut file) => {
            kdbx.write(&mut file).unwrap();
        }
        Err(e) => panic!("Error creating file: {}", e),
    };

    let kdbx = kdbx_rs::open(&full_path).unwrap();
    let key = CompositeKey::from_password(&database_password);
    let unlocked = kdbx.unlock(&key).map_err(|e| e.1.to_string());
    match unlocked {
        Ok(mut e) => {
            *database = e.database_mut().clone();
            Ok(())
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[tauri::command(async)]
fn shutdown(app: tauri::AppHandle) {
    //TODO: Save database before shutdown
    let database = app
        .state::<InternalState>()
        .database
        .lock()
        .unwrap()
        .clone();
    let state = app.state::<InternalState>();
    save_database(database, &state);
    app.exit(0)
}

#[tauri::command(async)]
fn lock(app: tauri::AppHandle) -> bool {
    //TODO: Save database before lock
    let database = app
        .state::<InternalState>()
        .database
        .lock()
        .unwrap()
        .clone();
    let state = app.state::<InternalState>();
    save_database(database, &state);
    true
    // app.exit(0)
}

#[tauri::command(async)]
fn upload_kdbx_database(
    state: State<InternalState>,
    path: String,
    password: String,
) -> Result<(), String> {
    let mut database_path = state.database_path.lock().unwrap();
    *database_path = path;

    let mut database_password = state.database_password.lock().unwrap();
    *database_password = password;

    let kdbx = kdbx_rs::open(&database_path.to_string()).unwrap();
    let key = CompositeKey::from_password(&database_password);
    let unlocked = kdbx.unlock(&key).map_err(|e| e.1.to_string());
    match unlocked {
        Ok(mut e) => {
            let mut database_unlocked = state.database.lock().unwrap();
            *database_unlocked = e.database_mut().clone();
            Ok(())
        }
        Err(e) => {
            return Err(e);
        }
    }
}
