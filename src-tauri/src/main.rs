// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_tz(id: String) -> Vec<String> {
    println!("Received get_tz request for ID: {}", id);
    iana_time_zone::get_timezone().ok().into_iter().collect::<Vec<String>>()
}

#[tauri::command]
fn get_cell_ids() -> Vec<String> {
    println!("Received get_cell_ids request");
    vec!{
        "ID1".to_string(),
        "ID2".to_string(),
        "ID3".to_string(),
        "ID4".to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_cell_ids, get_tz])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
