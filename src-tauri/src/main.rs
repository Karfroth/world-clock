// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod tray;
mod window;

use crate::commands::{get_cell_ids, get_tz, set_tz};
use crate::tray::get_show_menu;
use tauri::{AppHandle, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent};
use window::toggle_window_visibility;
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
  match event {
      SystemTrayEvent::MenuItemClick { id, .. } if id == "quit" => app.exit(0),
      SystemTrayEvent::MenuItemClick { id, .. } if id == "show_or_hide" => {
          let window = app.get_window("main");
          let tray_handler = app.tray_handle();
          if let Some(main_window) = window {
              toggle_window_visibility(&main_window, &tray_handler).unwrap();
          }
      },
      _ => {}
  }
}

fn main() {
    let show_menu = get_show_menu();
    let system_tray = SystemTray::new()
        .with_menu(show_menu);
    tauri::Builder::default()
        .setup(|app|{
          let mut shortcut_manager = app.global_shortcut_manager();
          let tray_handler = app.tray_handle();
          let window = app.get_window("main");
          if let Some(main_window) = window {
            #[cfg(target_os = "macos")]
            apply_vibrancy(&main_window, NSVisualEffectMaterial::HudWindow, None, Some(16.0)).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_blur(&main_window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            shortcut_manager.register("CmdOrCtrl+Shift+0", move || {
              toggle_window_visibility(&main_window, &tray_handler).unwrap();
            }).ok();
          }
          // }).ok();
          Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(handle_tray_event)
        .invoke_handler(tauri::generate_handler![get_cell_ids, get_tz, set_tz])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
        
}
