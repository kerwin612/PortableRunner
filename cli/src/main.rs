extern crate portable_desktop_env;
extern crate test_path;

use std::io::{stdin};
use std::env::{args};
use std::process::Command;
use std::path::{Path, Component};
use std::fs::{read_dir, remove_dir_all};
use portable_desktop_env::{mount, unmount};
use test_path::is_valid;

fn main() {
    let in_args: Vec<String> = args().collect();

    if in_args.len() < 2 {
        println!("\
        PortableDesktopCli [options] <Target Path> <Link Path> [Home name]\n\
            \toptions:\n\
            \t\t-f|--force: If the <Link Path> already exists, delete it and recreate it.\n\
            \targs:\n\
            \t\tTarget Path: Specifies the physical drive and path that you want to assign to a virtual drive.\n\
            \t\tLink Path: Specifies the virtual drive and path to which you want to assign a path.\n\
            \t\tHome name: The subdirectory name of the <Link Path> directory, Will be specified as the value of %HOME%, which defaults to [.home].\n\
        ");
    }

    let mut force = false;
    let mut a_index = 1;
    for (i, a) in in_args.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if a.starts_with("-") {
            match &*a.as_str() {
                "-f"|"--force" => force = true,
                _ => (),
            };
        } else {
            a_index = i;
            break;
        }
    }

    let mut tpath = String::new();
    if in_args.len() < a_index + 1 {
        println!("Target Path:");
        match stdin().read_line(&mut tpath) {
            Err(e) => panic!("problem typing the [Target Path]: {:?}", e),
            _ => (),
        }
        tpath.truncate(tpath.len() -2);
    } else {
        tpath = in_args[a_index].to_string();
    }

    let mut lpath = String::new();
    if in_args.len() < a_index + 2 {
        println!("Link Path:");
        match stdin().read_line(&mut lpath) {
            Err(e) => panic!("problem typing the [Link Path]: {:?}", e),
            _ => (),
        }
        lpath.truncate(lpath.len() -2);
    } else {
        lpath = in_args[a_index + 1].to_string();
    }

    if ! is_exists_dir(&tpath) {
        println!("Target Path [{}] does not exist.", &tpath);
        return;
    }

    if ! is_valid(&lpath) || ! has_disk(&lpath) || ! Path::new(&lpath).has_root() {
        println!("Link Path [{}] is invalid.", &lpath);
        return;
    }



    if force {
        match remove_dir_all(&lpath) {
            Err(e) => panic!("problem remove the [Link Path]: {:?}", e),
            _ => (),
        }
    }

    if is_exists_file(&lpath) {
        println!("Link Path [{}] is invalid.", &lpath);
        return;
    }

    let hpath = if in_args.len() == 4 { &in_args[3] } else { ".home" };

    if ! exists(&tpath, &lpath) {
        match mount(&tpath, &lpath, &hpath, true) {
            Err(e) => panic!("problem mounting the path: {:?}", e),
            _ => (),
        }
    }

    let home_path = format!("{}\\{}", lpath, hpath);

    println!("{} <<===>> {}", &lpath, &tpath);

    println!("%HOME% => {}", home_path);

    Command::new("CMD")
        .current_dir(home_path)
        .status()
        .expect("cmd command failed to start");

    unmount(&lpath);

}

fn is_exists_dir(path: &str) -> bool {
    let _path = Path::new(path);
    return _path.exists() && _path.is_dir();
}

fn is_exists_file(path: &str) -> bool {
    let _path = Path::new(path);
    return _path.exists() && _path.is_file();
}

fn exists(tpath: &str, lpath: &str) -> bool {
    let _lpath = Path::new(&lpath);
    if _lpath.exists() {
        let paths = read_dir(_lpath).unwrap();
        for p in paths {
            if Path::new(&format!("{}\\{}", &tpath, p.unwrap().file_name().to_str().unwrap())).exists() {
                return true;
            }
        }
    }
    return false;
}

fn has_disk(path: &str) -> bool {
    match Path::new(path).components().next().unwrap() {
        Component::Prefix(_p) => return true,
        _ => return false,
    }
}
