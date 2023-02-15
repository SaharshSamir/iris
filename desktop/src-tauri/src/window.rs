// use std::borrow::Borrow;

// use cocoa::{
//     appkit::{NSMainMenuWindowLevel, NSWindow},
//     base::id,
// };
// use enigo::Enigo;
use tauri::{App, AppHandle, GlobalShortcutManager, Manager, Monitor, Window, Wry};

fn make_window_transparent(window: &Window<Wry>) {}

#[tauri::command]
pub fn init_spotlight_behaviour(window: Window<Wry>) {}

pub fn show_window(window: &Window<Wry>) {}

pub fn hide_window(window: &Window<Wry>) {}

// //set the window above menubar level
// fn set_window_above_menubar(window: &Window<Wry>) {
//     let handle: id = window.ns_window().unwrap() as _;
//     unsafe { handle.setLevel_((NSMainMenuWindowLevel + 2).into()) }
// }

// fn get_window_with_mouse(window: &Window<Wry>) -> Option<Monitor> {
//     let mouse_location: (i32, i32) = Enigo::mouse_location();

//     let screens = window.available_monitors().unwrap();

//     let index = screens
//         .iter()
//         .position(|s| {
//             let size: LogicalSize<i32> = s.size().to_logical(s.scale_factor());
//             let in_x =
//                 (s.position().x..(s.position().x + size.width as i32)).contains(&cursor_location.0);
//             let in_y = (s.position().y..(s.position().y + size.height) as i32)
//                 .contains(&cursor_location.1);
//             return in_x && in_y;
//         })
//         .unwrap();
//     let screen = screens[index].borrow().clone();
//     return Some(screen);
// }
