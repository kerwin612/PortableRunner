use std::fs;
use std::env;
use std::path;
use std::time::{UNIX_EPOCH};
use std::io::{Write, Error, ErrorKind};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub shortcuts: Vec<Shortcut>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shortcut {
    pub key: String,
    pub cmd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_file_drop: Option<Vec<WithFileDrop>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithFileDrop {
    pub pattern: String,
    pub parameters: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments_required: Option<bool>,
}

const CFG_NAME: &str = ".pr.json";

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
            "withFileDrop": [
                {
                    "pattern": ".*",
                    "folderRequired": true,
                    "parameters": "/s /k cd /d \"{0}\""
                },
                {
                    "pattern": ".*\\.(bat|cmd)$",
                    "fileRequired": true,
                    "argumentsRequired": true,
                    "parameters": "/s /k \"{0}\""
                }
            ],
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
            "withFileDrop": [
                {
                    "pattern": ".*\\.(txt|log|out)$",
                    "fileRequired": true,
                    "parameters": "\"{0}\""
                }
            ],
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
            "argumentsRequired": true,
            "group": "open",
            "withFileDrop": [
                {
                    "pattern": ".*\\.(exe)$",
                    "fileRequired": true,
                    "parameters": "\"{0}\""
                }
            ],
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

pub fn generate_default_cfg() -> Result<bool, Error> {
    match env::var("HOME") {
        Ok(val) => {
            let home_path = path::Path::new(&val);
            if !(home_path.exists()) {
                fs::create_dir_all(home_path).unwrap();
            }
            let pr_path = home_path.join(CFG_NAME);
            if !(pr_path.exists()) {
                match fs::File::create(pr_path) {
                    Ok(mut file) => {
                        match file.write_all(DEFAULT_CFG.as_bytes()) {
                            Ok(_) => Ok(true),
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => Err(e),
                }
            } else {
                Ok(false)
            }
        },
        Err(_) => Err(Error::new(ErrorKind::NotFound, "invalid HOME")),
    }
}

pub fn epoch_cfg() -> Result<u128, Error> {
    match std::env::var("HOME") {
        Ok(val) => {
            let home_path = path::Path::new(&val);
            let pr_path = home_path.join(CFG_NAME);
            if pr_path.exists() {
                Ok(fs::metadata(pr_path).unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis())
            } else {
                Ok(0)
            }
        },
        Err(_) => Err(Error::new(ErrorKind::NotFound, "invalid HOME")),
    }
}

pub fn read_cfg() -> Result<Config, Error> {
    match std::env::var("HOME") {
        Ok(val) => {
            let home_path = path::Path::new(&val);
            let pr_path = home_path.join(CFG_NAME);
            let content = fs::read_to_string(&pr_path).unwrap();
            Ok(serde_json::from_str(&content)?)
        },
        Err(_) => Err(Error::new(ErrorKind::NotFound, "invalid HOME")),
    }
}

pub fn save_cfg(config: &Config) -> Result<(), Error> {
    match std::env::var("HOME") {
        Ok(val) => {
            let home_path = path::Path::new(&val);
            let pr_path = home_path.join(CFG_NAME);
            let mut buf = Vec::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            config.serialize(&mut ser).unwrap();

            let serialized = String::from_utf8(buf).unwrap();
            let mut file = fs::File::create(pr_path).unwrap();
            file.write_all(serialized.as_bytes()).unwrap();
            Ok(())
        },
        Err(_) => Err(Error::new(ErrorKind::NotFound, "invalid HOME")),
    }
}

pub fn add_shortcut(shortcut: Shortcut) -> Result<bool, Error> {
    let mut config = read_cfg().unwrap();
    config.shortcuts.push(shortcut);
    save_cfg(&config).unwrap();
    Ok(true)
}
