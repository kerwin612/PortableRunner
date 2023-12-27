// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod libs;

extern crate mount_dir;

use crate::libs::lnk::LnkInfo;
use crate::libs::config::Shortcut;

use std::fs;
use std::path::{Path};
use std::process::Command;
use std::env::{var, args, set_var};
use serde::{Serialize, Deserialize};
use std::os::windows::process::CommandExt;
use std::io::{Error, BufRead, BufReader, ErrorKind};
use tauri::{State, Manager, AppHandle, SystemTray, SystemTrayMenu, SystemTrayEvent, CustomMenuItem};
use tauri::api::shell::{open};

#[derive(Serialize, Deserialize)]
struct Storage {
    tpath: String,
    lpath: String,
    hpath: String,
}

#[tauri::command]
fn read_lnk(lnk: String) -> LnkInfo {
    libs::lnk::read_lnk(lnk)
}

#[tauri::command]
fn add_shortcut(shortcut: Shortcut) -> bool {
    libs::config::add_shortcut(shortcut).unwrap();
    return true;
}

#[tauri::command]
fn set_load(storage: State<Storage>) -> Storage {
    Storage { tpath: storage.tpath.clone(), lpath: storage.lpath.clone(), hpath: storage.hpath.clone() }
}

#[tauri::command]
fn set_save(set: Storage, _storage: State<Storage>, app: AppHandle) -> bool {
    match mount(&set.tpath, &set.lpath, &set.hpath, true) {
        Err(_e) => return false,
        Ok(_r) => {
            libs::config::generate_default_cfg().unwrap();
            let tray = app.tray_handle();
            tray.set_tooltip(&format!("PortableRunner ({} <=> {})", &set.lpath, &set.tpath)).unwrap();
            let window = app.get_window("main").unwrap();
            window.set_title(&format!("PortableRunner ({} <=> {})", &set.lpath, &set.tpath)).unwrap();
            return true;
        },
    }
}

#[tauri::command]
fn cmd_load() -> Vec<Shortcut> {
    let config = libs::config::read_cfg().unwrap();
    return config.shortcuts;
}

#[tauri::command]
fn cfg_epoch() -> u128 {
    return libs::config::epoch_cfg().unwrap();
}

#[tauri::command]
async fn cmd_runner(cmd_str: String) -> () {
    match std::env::var("HOME") {
        Ok(val) => {
            let temp_file = libs::utils::create_temp_file(&format!(r#"START "PortableRunner" {}"#, cmd_str));
            println!("[{}]: {}", &temp_file, &format!(r#"START "PortableRunner" /D {} {}"#, &val, cmd_str));
            Command::new("CMD").current_dir(&val).args(["/C", &temp_file]).creation_flags(0x08000000).status().unwrap();
            fs::remove_file(&temp_file).unwrap();
        },
        Err(_e) => (),
    }
}

fn mount(tpath: &str, lpath: &str, hpath: &str, force: bool) -> Result<bool, Error> {
    if ! Path::new(&tpath).exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("[{}] not found", &tpath)));
    }

    if Path::new(&lpath).exists() {
        if force {
            match fs::remove_dir_all(&lpath) {
                Err(e) => return Err(e),
                _ => (),
            }
        } else {
            return Err(Error::new(ErrorKind::AlreadyExists, format!("[{}] already exists", &lpath)));
        }
    }

    let _hpath = format!("{}\\{}", lpath, hpath);

    mount_dir::mount(tpath, lpath, force)?;

    let app_data = format!("{}\\AppData", &_hpath);
    let roaming_app_data = format!("{}\\Roaming", &app_data);
    let local_app_data = format!("{}\\Local", &app_data);
    let temp = format!("{}\\Temp", &local_app_data);

    set_var("PORTABLE_RUNNER_ENV_LINK_PATH", &lpath);
    set_var("PORTABLE_RUNNER_ENV_TARGET_PATH", &tpath);
    set_var("PORTABLE_RUNNER_HOST_TMP", var("TMP").unwrap_or("".to_string()));
    set_var("PORTABLE_RUNNER_HOST_TEMP", var("TEMP").unwrap_or("".to_string()));
    set_var("PORTABLE_RUNNER_HOST_HOME", var("HOME").unwrap_or("".to_string()));
    set_var("PORTABLE_RUNNER_HOST_APPDATA", var("APPDATA").unwrap_or("".to_string()));
    set_var("PORTABLE_RUNNER_HOST_HOMEPATH", var("HOMEPATH").unwrap_or("".to_string()));
    set_var("PORTABLE_RUNNER_HOST_HOMEDRIVE", var("HOMEDRIVE").unwrap_or("".to_string()));
    set_var("PORTABLE_RUNNER_HOST_USERPROFILE", var("USERPROFILE").unwrap_or("".to_string()));
    set_var("PORTABLE_RUNNER_HOST_LOCALAPPDATA", var("LOCALAPPDATA").unwrap_or("".to_string()));

    set_var("TMP", &temp);
    set_var("TEMP", &temp);
    set_var("HOME", &_hpath);
    set_var("HOMEPATH", &_hpath);
    set_var("USERPROFILE", &_hpath);
    set_var("HOMEDRIVE", libs::utils::get_disk(lpath));
    set_var("APPDATA", &roaming_app_data);
    set_var("LOCALAPPDATA", &local_app_data);

    let profile_path = libs::profile::get_profile().unwrap();
    if Path::new(&profile_path).exists() {
        let env_flag = format!(".env.{}.tmp", libs::utils::generate_random_string(32, "1234567890"));
        let output = Command::new("CMD").args(["/D", "/C", &profile_path, "&", "ECHO", &env_flag, "&", "SET"]).creation_flags(0x08000000).output().expect("process failed to execute");
        let reader = BufReader::new(&*output.stdout);
        let mut is_env = false;
        for line in reader.lines() {
            let l = line.unwrap();
            if ! is_env {
                if env_flag.eq(l.trim()) {
                    is_env = true;
                }
                continue;
            }
            match l.trim().split_once('=') {
                Some((key, value)) => {
                    set_var(key, value);
                }
                None => ()
            }
        }
    }

    return Ok(true);
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
        .add_item(CustomMenuItem::new("help".to_string(), "Help"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Exit"));
    SystemTray::new().with_menu(tray_menu)
}

fn tray_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            toggle_main_window(app);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "help" => {
                match open(&app.shell_scope(), "https://github.com/kerwin612/PortableRunner", None) {
                    Err(_) => (),
                    Ok(_) => ()
                }
            },
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
        .invoke_handler(tauri::generate_handler![read_lnk, set_load, set_save, cmd_load, cfg_epoch, cmd_runner, add_shortcut])
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
