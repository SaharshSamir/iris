#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::borrow::Borrow;

use reqwest::get;
use serde_json::{self, Value};
use tauri::{
    CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    WindowEvent,
};
use window::init_spotlight_behaviour;

mod window;

#[tauri::command]
async fn new_example(name: String) -> String {
    let ans = match get("http://127.0.0.1:6969/rspc/newex").await {
        Ok(res) => {
            let resp = res.json::<Value>().await.unwrap_or_default();

            println!("response: {:#?}", resp);
            "chala".to_owned()
        }
        Err(e) => {
            println!("{:?}", e);
            "Error occured".to_owned()
        }
    };
    return ans;
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            new_example,
            init_spotlight_behaviour
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(move |app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let window = app.get_window("main").unwrap();
            let windoww_for_focus_change = window.clone();
            let window_for_shortcut = window.clone();

            window.on_window_event(move |event| match event {
                WindowEvent::Focused(is_focused) => {
                    println!("focus change event: {}", is_focused);
                    if let false = is_focused {
                        windoww_for_focus_change.hide().unwrap();
                    }
                }
                _ => (),
            });
            println!("registering shortcut");
            match app
                .global_shortcut_manager()
                .register("CmdOrCtrl+Shift+Space", move || {
                    println!("insside register closure");
                    if window_for_shortcut.is_visible().unwrap() {
                        println!("is visible, hiding");
                        window.hide().unwrap();
                    } else {
                        println!("not visible, showing");
                        window_for_shortcut.show().unwrap();
                        window_for_shortcut
                            .set_focus()
                            .expect("could not set focus?");
                    }
                }) {
                Ok(_) => println!("Shortcut registered"),
                Err(_) => println!("Why are you tring again?"),
            }

            println!("shortcut registered");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
