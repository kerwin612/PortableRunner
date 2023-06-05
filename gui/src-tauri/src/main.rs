// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env::args;
use std::path::Path;
use std::io::{Error};
use std::process::Command;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::os::windows::process::CommandExt;
use tauri::{Manager, AppHandle, SystemTray, SystemTrayMenu, SystemTrayEvent, CustomMenuItem};
use tauri::State;
use serde_json::{Value};
use portable_runner_env::{mount};

#[derive(Serialize, Deserialize)]
struct Storage {
    tpath: String,
    lpath: String,
    hpath: String,
}

#[tauri::command]
fn set_load(storage: State<Storage>) -> Storage {
    Storage { tpath: storage.tpath.clone(), lpath: storage.lpath.clone(), hpath: storage.hpath.clone() }
}

#[tauri::command]
fn set_save(set: Storage, _storage: State<Storage>) -> bool {
    match do_mount(set) {
        Err(_e) => return false,
        Ok(_r) => return true,
    }
}

#[tauri::command]
fn cmd_load() -> Vec<Value> {
    match std::env::var("HOME") {
        Ok(val) => {
            let pd_path = format!("{}\\.pd.json", &val);
            if Path::new(&pd_path).exists() {
                let content = std::fs::read_to_string(&pd_path).unwrap();
                let config = serde_json::from_str::<HashMap<String, Value>>(&content).unwrap();
                return config["shortcuts"].as_array().unwrap().to_vec();
            }
        },
        Err(_e) => (),
    }
    return Vec::new();
}

#[tauri::command]
async fn cmd_runner(cmd_strs: Vec<String>) -> () {
    println!("{}", &format!("START /D %HOME% {} {}", cmd_strs[0].clone(), cmd_strs.strip_prefix(&[cmd_strs[0].clone()]).unwrap().join(" ")));
    Command::new("CMD").args(["/C", &format!("START /D %HOME% {} {}", cmd_strs[0].clone(), cmd_strs.strip_prefix(&[cmd_strs[0].clone()]).unwrap().join(" "))]).creation_flags(0x08000000).status().unwrap();
}

fn do_mount(storage: Storage) -> Result<bool, Error> {
    mount(&storage.tpath, &storage.lpath, &storage.hpath, true)
}

fn toggle_main_window(app: &AppHandle) {
    let window = app.get_window("main").unwrap();
    if let Ok(v) = window.is_visible() {
        if v {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

fn tray_menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Exit"));
    SystemTray::new().with_menu(tray_menu)
}

fn tray_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            toggle_main_window(app);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}

fn main() {
    let in_args: Vec<String> = args().collect();

    let mut tpath = String::new();
    if in_args.len() > 1 {
        tpath = in_args[1].to_string();
    }

    let mut lpath = String::new();
    if in_args.len() > 2 {
        lpath = in_args[2].to_string();
    }

    let mut hpath = String::new();
    if in_args.len() > 3 {
        hpath = in_args[3].to_string();
    }

    tauri::Builder::default()
        .manage(Storage { tpath, lpath, hpath })
        .system_tray(tray_menu())
        .on_system_tray_event(tray_handler)
        .invoke_handler(tauri::generate_handler![set_load, set_save, cmd_load, cmd_runner])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::WindowEvent {
                label: win_label,
                event: win_event,
                ..
            } => match win_event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    app.get_window(win_label.as_str()).unwrap().hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            },
            _ => {}
        });
}
