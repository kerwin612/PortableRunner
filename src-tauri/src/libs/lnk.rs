extern crate lnk;

use lnk::ShellLink;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LnkInfo {
    pub name: Option<String>,
    pub target: Option<String>,
    pub arguments: Option<String>,
}

pub fn read_lnk(lnk: String) -> LnkInfo {
    let shortcut = ShellLink::open(lnk).unwrap();
    match shortcut.link_info() {
        Some(link_info) => {
            return LnkInfo {
                name: shortcut.name().clone(),
                target: link_info.local_base_path().clone(),
                arguments: shortcut.arguments().clone(),
            }
        }
        &None => {
            return LnkInfo {
                name: None,
                target: None,
                arguments: None,
            }
        }
    }
}
