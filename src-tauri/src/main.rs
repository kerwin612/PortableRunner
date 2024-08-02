// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod libs;

extern crate mount_dir;

use crate::libs::config::Shortcut;
use crate::libs::lnk::LnkInfo;

use tauri_plugin_shell::ShellExt;
use serde::{Deserialize, Serialize};
use std::env::{args, set_var, var};
use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use tauri::{
    State,
    Manager,
    AppHandle,
    menu::{MenuBuilder, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
};

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
fn load_set(storage: State<Storage>) -> Storage {
    Storage {
        tpath: storage.tpath.clone(),
        lpath: storage.lpath.clone(),
        hpath: storage.hpath.clone(),
    }
}

#[tauri::command]
fn save_set(set: Storage, _storage: State<Storage>, app_handle: AppHandle) -> bool {
    match mount(&set.tpath, &set.lpath, &set.hpath, true) {
        Err(_e) => return false,
        Ok(_r) => {
            libs::config::generate_default_cfg().unwrap();
            app_handle.tray_by_id("main").expect("not found tray").set_tooltip(Some(&format!(
                "PortableRunner ({} <=> {})",
                &set.lpath, &set.tpath
            ))).unwrap();
            let window = app_handle.get_webview_window("main").unwrap();
            window
                .set_title(&format!(
                    "PortableRunner ({} <=> {})",
                    &set.lpath, &set.tpath
                )).unwrap();
            return true;
        }
    }
}

#[tauri::command]
fn load_cmds() -> Vec<Shortcut> {
    let config = libs::config::read_cfg().unwrap();
    return config.shortcuts;
}

#[tauri::command]
fn cfg_epoch() -> u128 {
    return libs::config::epoch_cfg().unwrap();
}

#[tauri::command]
async fn run_cmd(cmd_str: String) -> () {
    match std::env::var("HOME") {
        Ok(val) => {
            let temp_file =
                libs::utils::create_temp_file(&format!(r#"START "PortableRunner" {}"#, cmd_str));
            println!(
                "[{}]: {}",
                &temp_file,
                &format!(r#"START "PortableRunner" /D {} {}"#, &val, cmd_str)
            );
            Command::new("CMD")
                .current_dir(&val)
                .args(["/C", &temp_file])
                .creation_flags(0x08000000)
                .status()
                .unwrap();
            fs::remove_file(&temp_file).unwrap();
        }
        Err(_e) => (),
    }
}

fn mount(tpath: &str, lpath: &str, hpath: &str, force: bool) -> Result<bool, Error> {
    if !Path::new(&tpath).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("[{}] not found", &tpath),
        ));
    }

    if Path::new(&lpath).exists() {
        if force {
            match fs::remove_dir_all(&lpath) {
                Err(e) => return Err(e),
                _ => (),
            }
        } else {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("[{}] already exists", &lpath),
            ));
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
    set_var(
        "PORTABLE_RUNNER_HOST_TMP",
        var("TMP").unwrap_or("".to_string()),
    );
    set_var(
        "PORTABLE_RUNNER_HOST_TEMP",
        var("TEMP").unwrap_or("".to_string()),
    );
    set_var(
        "PORTABLE_RUNNER_HOST_HOME",
        var("HOME").unwrap_or("".to_string()),
    );
    set_var(
        "PORTABLE_RUNNER_HOST_APPDATA",
        var("APPDATA").unwrap_or("".to_string()),
    );
    set_var(
        "PORTABLE_RUNNER_HOST_HOMEPATH",
        var("HOMEPATH").unwrap_or("".to_string()),
    );
    set_var(
        "PORTABLE_RUNNER_HOST_HOMEDRIVE",
        var("HOMEDRIVE").unwrap_or("".to_string()),
    );
    set_var(
        "PORTABLE_RUNNER_HOST_USERPROFILE",
        var("USERPROFILE").unwrap_or("".to_string()),
    );
    set_var(
        "PORTABLE_RUNNER_HOST_LOCALAPPDATA",
        var("LOCALAPPDATA").unwrap_or("".to_string()),
    );

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
        let env_flag = format!(
            ".env.{}.tmp",
            libs::utils::generate_random_string(32, "1234567890")
        );
        let output = Command::new("CMD")
            .args([
                "/D",
                "/C",
                &profile_path,
                "&",
                "ECHO",
                &env_flag,
                "&",
                "SET",
            ])
            .creation_flags(0x08000000)
            .output()
            .expect("process failed to execute");
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let reader = BufReader::new(stdout_str.as_bytes());
        let mut is_env = false;
        for line in reader.lines() {
            let l = line.unwrap();
            if !is_env {
                if env_flag.eq(l.trim()) {
                    is_env = true;
                }
                continue;
            }
            match l.trim().split_once('=') {
                Some((key, value)) => {
                    set_var(key, value);
                }
                None => (),
            }
        }
    }

    return Ok(true);
}

fn toggle_main_window(app_handle: &AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    if let Ok(v) = window.is_visible() {
        if v {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
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
        .invoke_handler(tauri::generate_handler![
            add_shortcut,
            cfg_epoch,
            load_cmds,
            load_set,
            read_lnk,
            run_cmd,
            save_set
        ])
        .manage(Storage {
            tpath,
            lpath,
            hpath,
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            app.tray_by_id("main").expect("not found tray").set_menu(Some(
                MenuBuilder::new(app)
                    .item(&MenuItem::with_id(app, "help", "Help", true, None::<&str>)?)
                    .item(&MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?)
                    .build()?,
            ))?;
            app.on_menu_event(move |app, event| match event.id().as_ref() {
                "help" => {
                    match app.shell().open(
                        "https://github.com/kerwin612/PortableRunner",
                        None,
                    ) {
                        Err(_) => (),
                        Ok(_) => (),
                    }
                }
                "exit" => {
                    std::process::exit(0);
                }
                _ => {}
            });
            app.on_tray_icon_event(|tray, event| {
                if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                } = event
                {
                    toggle_main_window(tray.app_handle());
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
