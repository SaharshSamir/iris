#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{
    borrow::Borrow,
    error::Error,
    fs::{create_dir_all, read_to_string, write},
};

// use anyhow::Result;
use reqwest::get;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use tauri::{
    api::{file::read_string, path::app_config_dir},
    CustomMenuItem, GlobalShortcutManager, Manager, PathResolver, SystemTray, SystemTrayEvent,
    SystemTrayMenu, WindowEvent,
};
use window::init_spotlight_behaviour;
use window_shadows::set_shadow;
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

mod window;

#[derive(Serialize, Deserialize, Debug)]
struct DeviceInfo {
    name: String,
    device_type: String,
}

impl DeviceInfo {
    fn new(name: String) -> Self {
        return Self {
            name: name,
            device_type: String::from("Computer"),
        };
    }
}

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
            init_spotlight_behaviour,
            read_device_info,
            set_device_info
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
            set_shadow(&window, true).unwrap();

            // #[cfg(target_os = "macos")]
            // apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
            //     .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            // #[cfg(target_os = "windows")]
            // apply_blur(&window, Some((18, 18, 18, 125)))
            //     .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            //window.on_window_event(move |event| match event {
            //     WindowEvent::Focused(is_focused) => {
            //         println!("focus change event: {}", is_focused);
            //         if let false = is_focused {
            //             windoww_for_focus_change.hide().unwrap();
            //         }
            //     }
            //     _ => (),
            // });
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

#[tauri::command]
fn read_device_info(app: tauri::AppHandle) -> Result<String, String> {
    //
    //println!("hey");
    //1. Device information
    let path_resolver = app.path_resolver();
    let mut device_info_file = path_resolver.app_config_dir().unwrap();
    device_info_file.push("device_info.json");

    //create_dir_all(device_info_file.parent().unwrap()).expect("could not create dir");

    //let file: string = read_to_string(&device_info_file).unwrap_or_else(|e| {
    //    eprintln!("folder or file didn't exist, creating a new one: {:?}", e);
    //    //write(&device_info_file, "{}").expect("trouble creating file");

    //    return err("file doesn't exist".to_string());
    //});
    /*
    match read_to_string(&device_info_file) {
        Ok(file) => return Ok(file),
        Err(e) => return Err("Device info doesn't exist".to_string())
    };
    */
    return read_to_string(&device_info_file).map_err(|e| {
        eprint!("trouble reading file {:?}", e);
        return "device info not found".to_string();
    });
}

#[tauri::command]
fn set_device_info(app: tauri::AppHandle, device_name: String) {
    println!("device_name: {}", device_name);
    let path_resolver = app.path_resolver();
    let mut device_info_file = path_resolver.app_config_dir().unwrap();

    device_info_file.push("device_info.json");

    create_dir_all(device_info_file.parent().unwrap()).expect("could not create dir");
    let device_info = DeviceInfo::new(device_name);

    write(
        &device_info_file,
        serde_json::to_string_pretty(&device_info).unwrap(),
    )
    .unwrap();
}
