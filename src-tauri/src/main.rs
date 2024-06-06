// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;

use crate::commands::{get_cell_ids, get_tz, set_tz};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

fn main() {
    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
              position: _,
              size: _,
              ..
            } => {
              println!("system tray received a left click");
              if let Some(main_window) = app.get_window("main") {
                main_window.is_visible().and_then(|visible| {
                    if !visible { main_window.show().and_then(|_| main_window.set_focus()) } else { main_window.hide() }
                }).ok();
              }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![get_cell_ids, get_tz, set_tz])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
              event.window().hide().unwrap();
            //   println!("{}", event.window().label());
              api.prevent_close();
            }
            _ => {}
          })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
        
}
