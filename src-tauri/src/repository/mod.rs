use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

pub fn init() -> () {
    if !db_file_exists() {
        // init_database_schemas("keys.db")?;
        create_db_file();
    }
    init_database_schemas().unwrap();
}

fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file.
    fs::File::create(db_path).unwrap();
}

fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::exists(Path::new(&db_path))
}

fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/rustylock/database.sqlite"
}

pub fn init_connection() -> Result<Connection> {
    let path = get_db_path();
    let co = Connection::open(path).unwrap();
    Ok(co)
}

pub fn init_database_schemas() -> Result<Connection> {
    let co = init_connection().unwrap();
    init_databases(&co)?;
    Ok(co)
}

fn init_databases(co: &Connection) -> Result<()> {
    println!("Initializing database schemas...");
    co.execute(
        "CREATE TABLE keys (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            last_used_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            last_changed_at DATETIME DEFAULT CURRENT_TIMESTAMP
)",
        [],
    )?;

    //     co.execute(
    //         "CREATE TABLE IF NOT EXISTS passwords (
    //             id TEXT PRIMARY KEY,
    //             password TEXT NOT NULL,
    //             created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    //             updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    //             last_used_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    //             last_changed_at DATETIME DEFAULT CURRENT_TIMESTAMP
    // )",
    //         [],
    //     )?;
    Ok(())
}
