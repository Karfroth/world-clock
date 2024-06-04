// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::ProjectDirs;
use jammdb::DB;
use uuid::Uuid;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_tz(id: String) -> Vec<String> {
    println!("Received get_tz request for ID: {}", id);
    let val = get(id.clone());
    let asdf = val.get(0).map(|x| x.to_owned()).unwrap_or("IDK".to_string());
    println!("Returning {} for get_tz request for ID: {}", asdf, id);
    val
}

fn get(id: String) -> Vec<String> {
    let db = get_db().unwrap();
    let tx = db.tx(false).unwrap();
    let bucket = tx.get_bucket("tzs").unwrap();
    let tz = bucket.get(id);
    tz.map(|x| String::from_utf8(x.kv().value().to_owned()).unwrap()).into_iter().collect::<Vec<String>>()
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn set_tz(id: String, tz: String) -> Vec<String> {
    println!("Received set_tz request for ID: {}, tz: {}", id, tz);
    put(id, tz.clone()).unwrap();
    vec! { tz }
}

fn put(id: String, tz: String) -> Result<(), jammdb::Error> {
    let db = get_db()?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("tzs")?;
    bucket.put(id, tz)?;
    tx.commit()
}

#[tauri::command]
fn get_cell_ids() -> Vec<String> {
    println!("Received get_cell_ids request");
    let db = get_db().unwrap();
    let tx = db.tx(true).unwrap();
    let bucket = tx.get_or_create_bucket("tzs").unwrap();
    let keys = bucket.kv_pairs().map(|kv| String::from_utf8(kv.key().to_owned()).unwrap()).collect::<Vec<String>>();
    
    let mut count = 0;
    let ks = if keys.len() == 4 {
        keys
    } else {
        let ks = (0..(4-keys.len())).map(|_x| {
            let id = Uuid::new_v4();
            let _ = bucket.put(id.to_string(), iana_time_zone::get_timezone().unwrap().to_string());
            count += 1;
            id.to_string()
        }).collect::<Vec<String>>();
        ks
    };
    let _ = tx.commit();

    println!("Pushed {} entries", ks.len());
    ks
}

fn get_db() -> Result<DB, jammdb::Error> {
    let dir = ProjectDirs::from("com", "karfkim",  "world clock").unwrap();
    let db_path = dir.data_dir().to_owned().join("db");
    let _ = std::fs::create_dir_all(db_path.clone());
    let db_path_str = format!("{}/my-database.db", db_path.to_str().unwrap());
    println!("Trying to open: {}", db_path_str);
    DB::open(db_path_str)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_cell_ids, get_tz, set_tz])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
