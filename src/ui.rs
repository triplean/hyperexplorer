use std::path::PathBuf;
use eframe::egui;
use egui::include_image;
use opener;
use crate::indexing;

#[derive(Default)]
pub struct HyperExplorer {
    sel_disk: Option<PathBuf>,
    curr_dir: Option<PathBuf>,
    dir_changed: bool,
    is_root: bool,
    drives: Vec<PathBuf>,
    search: String
}

impl HyperExplorer {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        Self {
            sel_disk: None,
            curr_dir: None,
            dir_changed: false,
            is_root: true,
            drives: Vec::new(),
            search: String::new()
        }
    }

    fn show_files(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(current_dir) = &self.curr_dir {
            let entries = indexing::listentries(current_dir);

            if !self.is_root && !self.drives.contains(current_dir) {
                let up_btn = ui.button("..");
                if up_btn.double_clicked() {
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
                                    self.curr_dir = Some(path);
                                    self.dir_changed = true;
                                    self.is_root = false;
                                }
                            }
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
        let disks = indexing::list_disks();
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
                    self.curr_dir = Some(disk_path);
                    self.is_root = true;
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
                    ui.vertical_centered(|ui| {
                        let searchlbl = egui::RichText::new("Search:");
                        ui.label(searchlbl);
                        let searchbox = egui::TextEdit::singleline(&mut self.search);
                        ui.add(searchbox);
                    });
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