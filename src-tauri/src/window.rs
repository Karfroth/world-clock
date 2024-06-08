use tauri::{SystemTrayHandle, Window};
use crate::tray::{MenuTransition, update_show_hide_menu};

pub fn toggle_window_visibility(main_window: &Window, tray_handler: &SystemTrayHandle) -> Result<(), tauri::Error> {
    let visible = main_window.is_visible()?;
    let transition = if visible { MenuTransition::ToShow } else { MenuTransition::ToHide };
    update_show_hide_menu(&tray_handler, transition);
    if !visible {
        main_window.show()?;
        main_window.set_focus()
    } else {
        main_window.hide()
    }
}
