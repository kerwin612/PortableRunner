use std::env;
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path;

const DEFAULT_PROFILE: &str = r#"@ECHO OFF

::----------------------------------------------------------------------
:: PortableRunner profile script.
::----------------------------------------------------------------------

"#;

pub fn get_profile() -> Result<String, Error> {
    match env::var("HOME") {
        Ok(val) => {
            let home_path = path::Path::new(&val);
            if !(home_path.exists()) {
                fs::create_dir_all(home_path).unwrap();
            }
            let mut profile_path = home_path.join(".profile.cmd");
            if !profile_path.exists() {
                profile_path = home_path.join(".profile.bat");
            }
            if !profile_path.exists() {
                profile_path = home_path.join(".profile.cmd");
                match fs::File::create(profile_path.clone()) {
                    Ok(mut file) => {
                        file.write_all(DEFAULT_PROFILE.as_bytes())?;
                    }
                    Err(e) => return Err(e),
                };
            }
            return Ok(profile_path.into_os_string().into_string().unwrap());
        }
        Err(_) => Err(Error::new(ErrorKind::NotFound, "invalid HOME")),
    }
}
