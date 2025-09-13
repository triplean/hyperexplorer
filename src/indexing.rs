use std::path::{Path, PathBuf};
use std::fs;
use std::fs::{read_dir, DirEntry, ReadDir};
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

