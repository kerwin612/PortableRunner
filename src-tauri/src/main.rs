// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate mount_dir;
extern crate random_string;
extern crate lnk;

use std::fs;
use std::process::Command;
use std::collections::HashMap;
use std::path::{Component, Path};
use std::env::{var, args, set_var};
use serde::{Serialize, Deserialize};
use std::time::{UNIX_EPOCH};
use std::os::windows::process::CommandExt;
use std::io::{Write, Error, BufRead, BufReader, ErrorKind};
use tauri::{State, Manager, AppHandle, SystemTray, SystemTrayMenu, SystemTrayEvent, CustomMenuItem};
use tauri::api::shell::{open};
use serde_json::{Value};
use random_string::generate;
use lnk::ShellLink;

const DEFAULT_PROFILE: &str = r#"@ECHO OFF

::----------------------------------------------------------------------
:: PortableRunner profile script.
::----------------------------------------------------------------------

"#;

const DEFAULT_CFG: &str = r#"{
    "shortcuts": [
        {
            "key": "shutdown",
            "cmd": "shutdown /s /f /t 0",
            "style": "background: linear-gradient(0deg, darkred 0%, white 50%, white 50%, darkred 100%);"
        },
        {
            "key": "reboot",
            "cmd": "shutdown /r /f /t 0",
            "style": "background: linear-gradient(0deg, darkorange 0%, white 50%, white 50%, darkorange 100%);"
        },
        {
            "key": "cmd",
            "cmd": "cmd",
            "withFileDrop": {
                "pattern": ".*",
                "folderRequired": true,
                "parameters": "/s /k cd /d {0}"
            },
            "style": "background-color: black; color: white;"
        },
        {
            "key": "profile",
            "cmd": "notepad %HOME%/.profile.cmd",
            "style": "background-color: pink;"
        },
        {
            "key": "configuration",
            "cmd": "notepad %HOME%/.pr.json",
            "style": "background: linear-gradient(45deg, dodgerblue, whitesmoke);"
        },
        {
            "key": "edge",
            "cmd": "msedge",
            "group": "accessories",
            "style": "background: linear-gradient(45deg, darkcyan, greenyellow);"
        },
        {
            "key": "mstsc",
            "cmd": "mstsc",
            "group": "accessories",
            "style": "background-color: dodgerblue;"
        },
        {
            "key": "notepad",
            "cmd": "notepad",
            "group": "accessories",
            "withFileDrop": {
                "pattern": ".*",
                "fileRequired": true,
                "parameters": "{0}"
            },
            "style": "background-color: #5bc0de;"
        },
        {
            "key": "calculator",
            "cmd": "calc",
            "group": "accessories",
            "style": "background-color: #6e98bf;"
        },
        {
            "key": "paint",
            "cmd": "mspaint",
            "group": "accessories",
            "style": "background: linear-gradient(217deg, rgba(255,0,0,.8), rgba(255,0,0,0) 70.71%), linear-gradient(127deg, rgba(0,255,0,.8), rgba(0,255,0,0) 70.71%), linear-gradient(336deg, rgba(0,0,255,.8), rgba(0,0,255,0) 70.71%);"
        },
        {
            "key": "open",
            "cmd": "explorer",
            "parametersRequired": true,
            "group": "open",
            "style": "background-color: navajowhite;"
        },
        {
            "key": "open-home",
            "cmd": "explorer %HOME%",
            "group": "open",
            "style": "background: linear-gradient(45deg, yellowgreen, greenyellow);"
        },
        {
            "key": "open-link",
            "cmd": "explorer %PORTABLE_RUNNER_ENV_LINK_PATH%",
            "group": "open",
            "style": "background: linear-gradient(45deg, forestgreen, whitesmoke);"
        },
        {
            "key": "open-target",
            "cmd": "explorer %PORTABLE_RUNNER_ENV_TARGET_PATH%",
            "group": "open",
            "style": "background: linear-gradient(45deg, dodgerblue, bisque);"
        }
    ]
}"#;

#[derive(Serialize, Deserialize)]
struct Storage {
    tpath: String,
    lpath: String,
    hpath: String,
}

#[derive(Serialize, Deserialize)]
struct LnkInfo {
    name: Option<String>,
    target: Option<String>,
    arguments: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    shortcuts: Vec<Shortcut>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Shortcut {
    key: String,
    cmd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_file_drop: Option<WithFileDrop>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WithFileDrop {
    pattern: String,
    parameters: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_required: Option<bool>,
}

fn read_pr_file(path: &Path) -> Result<Config, serde_json::Error> {
    let content = fs::read_to_string(&path).unwrap();
    serde_json::from_str(&content)
}

fn write_pr_file(path: &Path, config: &Config) -> Result<(), serde_json::Error> {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    config.serialize(&mut ser).unwrap();

    let serialized = String::from_utf8(buf).unwrap();
    let mut file = fs::File::create(path).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    Ok(())
}

#[tauri::command]
fn read_lnk(lnk: String) -> LnkInfo {
    let shortcut = ShellLink::open(lnk).unwrap();
    match shortcut.link_info() {
        Some(link_info) => {
            return LnkInfo {
                name: shortcut.name().clone(),
                target: link_info.local_base_path().clone(),
                arguments: shortcut.arguments().clone(),
            }
        },
        &None => return LnkInfo {
            name: None,
            target: None,
            arguments: None,
        },
    }
}

#[tauri::command]
fn add_shortcut(shortcut: Shortcut) -> bool {
    match std::env::var("HOME") {
        Ok(val) => {
            let pr_path_str = format!("{}\\.pr.json", &val);
            let pr_path = Path::new(&pr_path_str);
            let mut config = read_pr_file(&pr_path).unwrap();

            config.shortcuts.push(shortcut);

            write_pr_file(&pr_path, &config).unwrap();
            return true;
        },
        Err(_) => return false,
    }
}

#[tauri::command]
fn set_load(storage: State<Storage>) -> Storage {
    Storage { tpath: storage.tpath.clone(), lpath: storage.lpath.clone(), hpath: storage.hpath.clone() }
}

#[tauri::command]
fn set_save(set: Storage, _storage: State<Storage>, app: AppHandle) -> bool {
    match mount(Storage { tpath: set.tpath.clone(), lpath: set.lpath.clone(), hpath: set.hpath.clone() }) {
        Err(_e) => return false,
        Ok(_r) => {
            generate_default_cfg().unwrap();
            let tray = app.tray_handle();
            tray.set_tooltip(&format!("PortableRunner ({} <=> {})", &set.lpath, &set.tpath)).unwrap();
            let window = app.get_window("main").unwrap();
            window.set_title(&format!("PortableRunner ({} <=> {})", &set.lpath, &set.tpath)).unwrap();
            return true;
        },
    }
}

#[tauri::command]
fn cmd_load() -> Vec<Value> {
    match std::env::var("HOME") {
        Ok(val) => {
            let pr_path = format!("{}\\.pr.json", &val);
            if Path::new(&pr_path).exists() {
                let content = fs::read_to_string(&pr_path).unwrap();
                let config = serde_json::from_str::<HashMap<String, Value>>(&content).unwrap();
                return config["shortcuts"].as_array().unwrap().to_vec();
            }
        },
        Err(_e) => (),
    }
    return Vec::new();
}

#[tauri::command]
fn cfg_epoch() -> u128 {
    match std::env::var("HOME") {
        Ok(val) => {
            let pr_path = format!("{}\\.pr.json", &val);
            if Path::new(&pr_path).exists() {
                return fs::metadata(pr_path).unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis();
            }
        },
        Err(_e) => (),
    }
    return 0;
}

#[tauri::command]
async fn cmd_runner(cmd_str: String) -> () {
    match std::env::var("HOME") {
        Ok(val) => {
            let temp_file = create_temp_file(&format!(r#"START "PortableRunner" {}"#, cmd_str));
            println!("[{}]: {}", &temp_file, &format!(r#"START "PortableRunner" /D {} {}"#, &val, cmd_str));
            Command::new("CMD").current_dir(&val).args(["/C", &temp_file]).creation_flags(0x08000000).status().unwrap();
            fs::remove_file(&temp_file).unwrap();
        },
        Err(_e) => (),
    }
}

fn create_temp_file(text: &str) -> String {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(&format!(".pr.tmp.{}.cmd", generate(16, "1234567890")));

    let mut file = fs::File::create(&file_path).unwrap();
    write!(file, "{}", text).unwrap();

    file_path.to_str().unwrap().to_string()
}

fn mount(storage: Storage) -> Result<bool, Error> {
    do_mount(&storage.tpath, &storage.lpath, &storage.hpath, true)
}

fn do_mount(tpath: &str, lpath: &str, hpath: &str, force: bool) -> Result<bool, Error> {

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
    set_var("HOMEDRIVE", get_disk(lpath));
    set_var("APPDATA", &roaming_app_data);
    set_var("LOCALAPPDATA", &local_app_data);

    let mut profile_path = format!("{}\\.profile.cmd", &_hpath);
    if ! Path::new(&profile_path).exists() {
        profile_path = format!("{}\\.profile.bat", &_hpath);
    }
    if ! Path::new(&profile_path).exists() {
        profile_path = format!("{}\\.profile.cmd", &_hpath);
        generate_default_profile(&profile_path).unwrap();
    }
    if Path::new(&profile_path).exists() {
        let env_flag = format!(".env.{}.tmp", generate(32, "1234567890"));
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

fn get_disk(path: &str) -> &str {
    match Path::new(path).components().next().unwrap() {
        Component::Prefix(prefix_component) => {
            return prefix_component.as_os_str().to_str().unwrap();
        }
        _ => unreachable!(),
    }
}

fn generate_default_cfg() -> Result<bool, Error> {
    match std::env::var("HOME") {
        Ok(val) => {
            let home_path = Path::new(&val);
            if !(home_path.exists()) {
                fs::create_dir_all(home_path).unwrap();
            }
            let pr_path = home_path.join(".pr.json");
            if !(pr_path.exists()) {
                match fs::File::create(pr_path) {
                    Ok(mut file) => {
                        match file.write_all(DEFAULT_CFG.as_bytes()) {
                            Ok(_) => Ok(true), // File created and written successfully
                            Err(e) => Err(e), // Error occurred while writing to the file
                        }
                    },
                    Err(e) => Err(e), // Error occurred while creating the file
                }
            } else {
                Ok(false) // File already exists
            }
        },
        Err(_) => Err(Error::new(ErrorKind::NotFound, "invalid HOME")), // Error occurred while getting the HOME environment variable
    }
}

fn generate_default_profile(profile_path: &str) -> Result<bool, Error> {
    match fs::File::create(profile_path) {
        Ok(mut file) => {
            match file.write_all(DEFAULT_PROFILE.as_bytes()) {
                Ok(_) => Ok(true), // File created and written successfully
                Err(e) => Err(e), // Error occurred while writing to the file
            }
        },
        Err(e) => Err(e), // Error occurred while creating the file
    }
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
