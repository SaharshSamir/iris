#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use reqwest::get;
use serde_json::{self, Value};
use std::collections::HashMap;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![new_example])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
