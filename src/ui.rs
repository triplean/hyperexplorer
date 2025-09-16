use std::fs;
use std::path::PathBuf;
use clipboard_rs::{Clipboard, ClipboardContext};
use eframe::egui;
use std::path;
use opener;
use crate::filesystem;

#[derive(Default)]
pub struct HyperExplorer {
    sel_disk: Option<PathBuf>,
    curr_dir: Option<PathBuf>,
    dir_changed: bool,
    is_root: bool,
    drives: Vec<PathBuf>,
    search: String,
    curr_dir_text: String,
}

impl HyperExplorer {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        Self {
            sel_disk: None,
            curr_dir: None,
            dir_changed: false,
            is_root: true,
            drives: Vec::new(),
            search: String::new(),
            curr_dir_text: String::new()
        }
    }

    fn show_files(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(current_dir) = &self.curr_dir {
            let entries = filesystem::listentries(current_dir);

            if !self.is_root && !self.drives.contains(current_dir) {
                let up_btn = ui.button("<- Back");
                if up_btn.clicked() {
                    if let Some(parent) = current_dir.parent() {
                        self.curr_dir = Some(parent.to_path_buf());
                        self.dir_changed = true;
                    }
                }
            }

            match entries {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            // The icons are embedded in the binary, so the icons folder packed with the binary in GH Actions does nothing
                            // I spent hours trying to show an image from the icons folder, but I can't find what I want
                            let mut img = egui::Image::new(egui::include_image!("icons/unk_file.png"));
                            if entry.path().is_dir() {
                                img = egui::Image::new(egui::include_image!("icons/folder.png"));
                            }
                            let entrybttn = egui::Button::image_and_text(img, file_name);
                            let res = ui.add(entrybttn);
                            if res.double_clicked() {
                                let path = entry.path();
                                if path.is_file() {
                                    if let Err(e) = opener::open(&path) {
                                        eprintln!("Error opening file: {}", e);
                                    }
                                } else {
                                    self.curr_dir = Some(path.clone());
                                    self.dir_changed = true;
                                    self.is_root = false;
                                    self.curr_dir_text = path.to_string_lossy().to_string();
                                }
                            }

                            res.context_menu(|ui| {
                                if ui.button("Open").clicked() {
                                    let path = entry.path();
                                    if path.is_file() {
                                        if let Err(e) = opener::open(&path) {
                                            eprintln!("Error opening file: {}", e);
                                        }
                                    } else {
                                        self.curr_dir = Some(path);
                                        self.dir_changed = true;
                                        self.is_root = false;
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
                                                            if let Some(cd) = &self.curr_dir {
                                                                let pasted = filesystem::paste(&path, cd);
                                                                if pasted {
                                                                    self.dir_changed = true;
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
                                        Ok(clipboard) => {
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
                                }

                                if ui.button("Cancel").clicked() {
                                    ui.close();
                                }
                            });
                        }
                    }
                },
                Err(e) => {
                    ui.label(egui::RichText::new(format!("Error reading directory: {}", e))
                        .color(egui::Color32::RED));
                }
            }
        }
    }

    fn show_disks(&mut self, ui: &mut egui::Ui) {
        let disks = filesystem::list_disks();
        ui.horizontal(|ui| {
            for disk in &disks {
                let disk_path = disk.mount_point().to_owned();
                let mut disk_name = String::new();
                if disk.name().to_string_lossy().is_empty() {
                    disk_name = disk_path.to_string_lossy().to_string();
                } else {
                    disk_name = disk.name().to_string_lossy().to_string();
                }
                self.drives.push(disk_path.clone());

                let response = ui.add(
                    egui::Button::new(disk_name)
                );

                if response.clicked() {
                    self.sel_disk = Some(disk_path.clone());
                    self.curr_dir = Some(disk_path.clone());
                    self.is_root = true;
                    self.curr_dir_text = disk_path.to_string_lossy().to_string();
                }
            }
        });
    }
}

impl eframe::App for HyperExplorer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let header = egui::RichText::new("HyperExplorer").size(24.0);
                    ui.label(header);
                    ui.add(egui::TextEdit::singleline(&mut self.curr_dir_text).interactive(false));
                });

                self.show_disks(ui);

                if let Some(_) = &self.curr_dir {
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            self.show_files(ui, ctx);
                        });
                }
            });
        });

        if self.dir_changed {
            self.dir_changed = false;
            ctx.request_repaint();
        }
    }
}