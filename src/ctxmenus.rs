use std::fs::DirEntry;
use std::path;
use clipboard_rs::{Clipboard, ClipboardContext};
use eframe::egui::Response;
use crate::filesystem;

pub fn file_context(
    entry: &mut DirEntry, 
    mut res: Response, 
    curr_dir: &mut Option<std::path::PathBuf>,
    dir_changed: &mut bool,
    is_root: &mut bool,
) {
    res.context_menu(|ui| {
        if ui.button("Open").clicked() {
            let path = entry.path();
            if path.is_file() {
                if let Err(e) = opener::open(&path) {
                    eprintln!("Error opening file: {}", e);
                }
            } else {
                *curr_dir = Some(path);
                *dir_changed = true;
                *is_root = false;
            }
            ui.close();
        }

        let clip = ClipboardContext::new();
        match clip {
            Ok(clipboard) => {
                let cliptext = clipboard.get_text();
                match cliptext {
                    Ok(text) => {
                        let clean_text = text.trim().trim_matches('"').trim();
                        let path = path::PathBuf::from(&clean_text);
                        match path.exists() {
                            true => {
                                if ui.button("Paste").clicked() {
                                    if let Some(cd) = &curr_dir {
                                        let pasted = filesystem::paste(&path, cd);
                                        if pasted {
                                            *dir_changed = true;
                                        } else {
                                            eprintln!("Error while pasting the file");
                                        }
                                    } else {
                                        eprintln!("There's not selected directory");
                                    }
                                }
                            },
                            false => {
                                eprintln!("The path doesn't exist or it's not accessible: {:?}", path);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error opening file: {}", e);
                        ui.close();
                        return;
                    }
                }
            },
            Err(e) => {
                eprintln!("Error accessing clipboard: {}", e);
                ui.close();
                return;
            }
        }

        if ui.button("Copy").clicked() {
            let clipb = ClipboardContext::new();
            match clipb {
                Ok(mut clipboard) => {
                    clipboard.set_text(entry.path().to_string_lossy().to_string()).expect("Can't write to clipboard");
                },
                Err(e) => {
                    eprintln!("Error accessing clipboard: {}", e);
                    ui.close();
                    return;
                }
            }
            ui.close();
        }

        let delbttn = ui.button("Delete Permanently");
        if delbttn.clicked() {
            if entry.path().is_file() {
                filesystem::delete_file(&entry.path());
            } else {
                filesystem::delete_path(&entry.path());
            }
            *dir_changed = true;
            ui.close();
        }

        if ui.button("Cancel").clicked() {
            ui.close();
        }
    });
}