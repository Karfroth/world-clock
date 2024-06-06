use crate::db::{get, get_or_create_tzs, put};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn get_tz(id: String) -> Vec<String> {
    println!("Received get_tz request for ID: {}", id);
    let val = get(id.clone());
    let asdf = val.get(0).map(|x| x.to_owned()).unwrap_or("IDK".to_string());
    println!("Returning {} for get_tz request for ID: {}", asdf, id);
    val
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn set_tz(id: String, tz: String) -> Vec<String> {
    println!("Received set_tz request for ID: {}, tz: {}", id, tz);
    put(id, tz.clone()).unwrap();
    vec! { tz }
}

#[tauri::command]
pub fn get_cell_ids() -> Vec<String> {
    println!("Received get_cell_ids request");
    get_or_create_tzs()
}