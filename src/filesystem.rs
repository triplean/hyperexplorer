use std::fs;
use std::path::PathBuf;
use std::fs::{read_dir, DirEntry, ReadDir};
use std::io::Read;
use sysinfo::Disks;

pub fn list_disks() -> Disks {
    Disks::new_with_refreshed_list()
}

pub fn listentries(dir: &PathBuf) -> Result<Vec<DirEntry>, String> {
    let mut items: Option<ReadDir> = None;
    match read_dir(dir) {
        Ok(dir) => items = Some(dir),
        Err(e) => return Err(e.to_string())
    }
    let mut entries = Vec::new();
    if items.is_some() {
        for item in items.unwrap() {
            entries.push(item.unwrap());
        }
    }

    Ok(entries)
}

pub fn delete_file(path: &PathBuf) -> bool{
    let res = fs::remove_file(path);
    match res {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error while deleting the file: {}", e);
            false
        }
    }
}

pub fn delete_path(path: &PathBuf) -> bool{
    let res = fs::remove_dir(path);
    match res {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error while deleting the folder: {}", e);
            false
        }
    }
}

pub fn paste(origin: &PathBuf, destination: &PathBuf) -> bool {
    let file_name = match origin.file_name() {
        Some(name) => name,
        None => {
            eprintln!("We couldn't get the original file name");
            return false;
        }
    };
    
    let mut dest_path = destination.clone();
    dest_path.push(file_name);

    match fs::copy(origin, &dest_path) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error while copying the file: {}", e);
            false
        }
    }
}