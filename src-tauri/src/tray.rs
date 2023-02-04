use tauri::{SystemTray, SystemTrayMenu, CustomMenuItem, SystemTrayEvent};
use tauri::AppHandle;
use tauri::Manager;

pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quite".to_string(), "Quite"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn handle(app: &AppHandle, event: SystemTrayEvent) {

    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = match app.get_window("main") {
                Some(window) => match window.is_visible().expect("winvis") {
                    true => {
                        window.hide().expect("winhide");
                        return;
                    }
                    false => window,
                },
                None => return,
            };
            window.set_focus().unwrap();
        },
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {

        },
        SystemTrayEvent::MenuItemClick{id, ..} => match id.as_str() {
            "quite" => {
                std::process::exit(0);
            },
            _ => {}
        },
        _ => {}
    }
}