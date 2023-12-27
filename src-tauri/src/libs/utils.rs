extern crate random_string;

use std::fs;
use std::env;
use std::io::{Write};
use std::path::{Component, Path};
use random_string::generate;

pub fn get_disk(path: &str) -> &str {
    match Path::new(path).components().next().unwrap() {
        Component::Prefix(prefix_component) => {
            return prefix_component.as_os_str().to_str().unwrap();
        }
        _ => unreachable!(),
    }
}

pub fn create_temp_file(text: &str) -> String {
    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join(&format!(".pr.tmp.{}.cmd", generate_random_string(16, "1234567890")));

    let mut file = fs::File::create(&file_path).unwrap();
    write!(file, "{}", text).unwrap();

    file_path.to_str().unwrap().to_string()
}

pub fn generate_random_string(length: usize, text: &str) -> String {
    generate(length, text)
}
