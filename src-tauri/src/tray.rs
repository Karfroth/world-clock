use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

pub fn get_show_menu() -> SystemTrayMenu {
    println!("Show Menu");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show_or_hide".to_string(), "Show");
    SystemTrayMenu::new()
      .add_item(quit)
      .add_native_item(SystemTrayMenuItem::Separator)
      .add_item(show)
}

pub enum MenuTransition {
    ToShow,
    ToHide,
}

#[inline]
pub fn update_show_hide_menu(tray_handler: &tauri::SystemTrayHandle, transition: MenuTransition) {
    let title = match transition {
        MenuTransition::ToShow => "Show",
        MenuTransition::ToHide => "Hide",
    };
    let item = tray_handler.get_item("show_or_hide");
    println!("Updating title to {}", title);
    item.set_title(title).unwrap()
}