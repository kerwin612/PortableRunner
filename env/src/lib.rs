extern crate mount_dir;
extern crate random_string;

use std::process::Command;
use std::env::{var, set_var};
use std::fs::{remove_dir_all};
use std::path::{Component, Path};
use std::os::windows::process::CommandExt;
use std::io::{Error, BufRead, BufReader, ErrorKind};

use random_string::generate;

pub fn mount(tpath: &str, lpath: &str, hpath: &str, force: bool) -> Result<bool, Error> {

    if ! Path::new(&tpath).exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("[{}] not found", &tpath)));
    }

    if Path::new(&lpath).exists() {
        if force {
            match remove_dir_all(&lpath) {
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

pub fn unmount(link: &str) -> bool {
    return mount_dir::unmount(link);
}

fn get_disk(path: &str) -> &str {
    match Path::new(path).components().next().unwrap() {
        Component::Prefix(prefix_component) => {
            return prefix_component.as_os_str().to_str().unwrap();
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod it_works {
    use super::*;
    use std::env;

    #[test]
    fn test_for_mount() {
        assert_eq!(mount(env::current_dir().unwrap().as_path().to_str().unwrap(), "T:\\work", "home", true).unwrap(), true);
    }

    #[test]
    fn test_for_unmount() {
        assert_eq!(unmount("T:\\work"), true);
    }
}
