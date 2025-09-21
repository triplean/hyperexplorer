// What you're about to see it's REALLY REALLY bad.
// I think I'm going to end up dedicating an update for cleaning this specific file
// Also, I'll try to write comments more often so you don't get THAT lost

use std::path::PathBuf;
use eframe::egui;
use opener;
use crate::{filesystem, icons};
use crate::search::FileSearcher;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;
use egui::Visuals;
use crate::ctxmenus;

/// Messages used for communication between the main thread and worker threads
#[derive(Debug, Clone)]
pub enum ThreadMessage {
    /// Signals that the indexing process has started
    IndexingStarted,
    /// Reports the current progress of the indexing operation
    IndexingProgress(usize),
    /// Signals that indexing has completed successfully
    IndexingCompleted(usize),
    /// Reports an error that occurred during indexing
    IndexingError(String),
    
    /// Signals that a search operation has started
    SearchStarted,
    /// Signals that a search operation has completed
    SearchCompleted(Vec<crate::search::FileResult>),
    /// Reports an error that occurred during search
    SearchError(String),
}

/// Main application state for HyperExplorer
/// This struct holds all the state needed for the file explorer UI,
/// including the current directory, search state, and thread management.
#[derive(Default)]
pub struct HyperExplorer {
    // Navigation state
    sel_disk: Option<PathBuf>,          // Currently selected disk/drive
    curr_dir: Option<PathBuf>,          // Current directory being viewed
    dir_changed: bool,                  // Flag indicating if directory changed
    is_root: bool,                      // Whether we're at the root directory
    drives: Vec<PathBuf>,               // List of available drives/disks
    
    // Search state
    search: String,                     // Current search query
    curr_dir_text: String,              // Textual representation of current directory
    search_engine: Arc<Mutex<FileSearcher>>,  // Thread-safe search engine
    is_searching: bool,                 // Whether a search is in progress

    // Indexing state
    is_indexing: bool,                  // Whether indexing is in progress
    indexing_start: Option<Instant>,    // When the current indexing operation started
    indexed_files: usize,               // Number of files indexed so far
    status_msg: String,                 // Current status message to display

    // Thread communication
    thread_receiver: Option<mpsc::Receiver<ThreadMessage>>,  // Channel for receiving thread messages

    // Search results
    search_results: Vec<crate::search::FileResult>,  // Current search results
    show_search_results: bool,           // Whether to show search results
}

impl HyperExplorer {
    /// Creates a new instance of HyperExplorer with the given search engine
    pub fn new(_cc: &eframe::CreationContext, search_engine: FileSearcher) -> Self {
        Self {
            sel_disk: None,
            curr_dir: None,
            dir_changed: false,
            is_root: true,
            drives: Vec::new(),
            search: String::new(),
            curr_dir_text: String::new(),
            search_engine: Arc::new(Mutex::new(search_engine)),
            is_indexing: false,
            is_searching: false,
            indexing_start: None,
            indexed_files: 0,
            status_msg: String::new(),
            thread_receiver: None,
            search_results: Vec::new(),
            show_search_results: false,
        }
    }

    /// Starts the file indexing process in a separate thread
    fn start_indexing(&mut self, path: PathBuf, ctx: &egui::Context) {
        if self.is_indexing {
            return;
        }

        self.is_indexing = true;
        self.indexing_start = Some(Instant::now());
        self.indexed_files = 0;
        self.status_msg = "Starting indexing...".to_string();

        let (sender, receiver) = mpsc::channel();
        self.thread_receiver = Some(receiver);

        let search_engine = Arc::clone(&self.search_engine);
        let ctx_clone = ctx.clone();

        thread::spawn(move || {
            let _ = sender.send(ThreadMessage::IndexingStarted);

            let mut local_searcher = FileSearcher::new();

            match local_searcher.index(&path.to_string_lossy(), true) {
                Ok(file_count) => {
                    if let Ok(mut main_searcher) = search_engine.lock() {
                        *main_searcher = local_searcher;
                    }
                    let _ = sender.send(ThreadMessage::IndexingCompleted(file_count));
                }
                Err(e) => {
                    let _ = sender.send(ThreadMessage::IndexingError(e.to_string()));
                }
            }

            ctx_clone.request_repaint();
        });
    }

    /// Processes messages received from worker threads
    /// This method handles all inter-thread communication and updates the UI state accordingly.
    fn check_thread_messages(&mut self, ctx: &egui::Context) {
        let mut should_clear_receiver = false;
        let mut messages_to_process = Vec::new();

        if let Some(receiver) = &self.thread_receiver {
            while let Ok(message) = receiver.try_recv() {
                messages_to_process.push(message);
            }
        }

        let mut should_clear_receiver = false;
        for message in &messages_to_process {
            match message {
                ThreadMessage::IndexingStarted => {
                    self.status_msg = "Indexing files...".to_string();
                }
                ThreadMessage::IndexingProgress(count) => {
                    self.indexed_files = *count;
                    if let Some(start) = self.indexing_start {
                        let elapsed = start.elapsed().as_millis();
                        self.status_msg = format!("Indexing... {} files ({} ms)", count, elapsed);
                    }
                }
                ThreadMessage::IndexingCompleted(total) => {
                    self.is_indexing = false;
                    self.indexed_files = *total;
                    if let Some(start) = self.indexing_start {
                        let elapsed = start.elapsed().as_millis();
                        self.status_msg = format!("Indexed {} files in {} ms", total, elapsed);
                    }
                    should_clear_receiver = true;

                    let ctx_clone = ctx.clone();
                    thread::spawn(move || {
                        thread::sleep(std::time::Duration::from_secs(3));
                        ctx_clone.request_repaint();
                    });
                }
                ThreadMessage::IndexingError(error) => {
                    self.is_indexing = false;
                    self.status_msg = format!("Indexing error: {}", error);
                    should_clear_receiver = true;
                }
                ThreadMessage::SearchStarted => {
                    self.is_searching = true;
                    self.status_msg = "Searching...".to_string();
                }
                ThreadMessage::SearchCompleted(results) => {
                    self.is_searching = false;
                    self.search_results = results.clone();
                    self.show_search_results = !self.search_results.is_empty();
                    self.status_msg.clear();
                }
                ThreadMessage::SearchError(error) => {
                    self.is_searching = false;
                    self.status_msg = format!("Search error: {}", error);
                }
            }
            ctx.request_repaint();
        }

        if should_clear_receiver {
            self.thread_receiver = None;
        }

        if !self.is_indexing && self.status_msg.contains("Indexed") {
            if let Some(start) = self.indexing_start {
                if start.elapsed().as_secs() > 3 {
                    self.status_msg.clear();
                    self.indexing_start = None;
                }
            }
        }
    }

    /// Performs a search operation based on the current search query
    fn perform_search(&mut self, _ctx: &egui::Context) {
        if self.search.trim().is_empty() {
            self.search_results.clear();
            self.show_search_results = false;
            return;
        }

        if self.is_searching {
            return;
        }

        if let Ok(searcher) = self.search_engine.lock() {
            self.search_results = searcher.search(&self.search, 50);
            self.show_search_results = !self.search_results.is_empty();
        }
    }

    /// Renders the file browser interface
    fn show_files(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(current_dir) = &self.curr_dir {
            let entries = filesystem::listentries(current_dir);

            if !self.is_root && !self.drives.contains(current_dir) {
                let up_btn = ui.button("<- Back");
                if up_btn.clicked() {
                    if let Some(parent) = current_dir.parent() {
                        self.curr_dir = Some(parent.to_path_buf());
                        self.dir_changed = true;
                        self.show_search_results = false;
                    }
                }
            }

            match entries {
                Ok(entries) => {
                    for mut entry in entries {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            let img = if entry.path().is_dir() {
                                egui::Image::new(egui::include_image!("icons/folder.png"))
                            } else {
                                icons::get_file_icon(&file_name)
                            };

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
                                    self.show_search_results = false;
                                }
                            }

                            ctxmenus::file_context(
                                &mut entry, 
                                res, 
                                &mut self.curr_dir, 
                                &mut self.dir_changed, 
                                &mut self.is_root
                            );
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


    /// Displays the available disks/drives for navigation
    fn show_disks(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let disks = filesystem::list_disks();
        ui.horizontal(|ui| {
            for disk in &disks {
                let disk_path = disk.mount_point().to_owned();
                let disk_name = if disk.name().to_string_lossy().is_empty() {
                    disk_path.to_string_lossy().to_string()
                } else {
                    disk.name().to_string_lossy().to_string()
                };
                self.drives.push(disk_path.clone());

                let mut button = egui::Button::new(&disk_name);

                if self.is_indexing {
                    button = button.fill(egui::Color32::GRAY);
                }

                let response = ui.add(button);

                if response.clicked() && !self.is_indexing {
                    self.sel_disk = Some(disk_path.clone());
                    self.curr_dir = Some(disk_path.clone());
                    self.is_root = true;
                    self.curr_dir_text = disk_path.clone().to_string_lossy().to_string();
                    self.show_search_results = false;

                    self.start_indexing(disk_path, ctx);
                }
            }
        });
    }
}


// App main loop
impl eframe::App for HyperExplorer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.check_thread_messages(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().visuals = Visuals::dark();
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let header = egui::RichText::new("HyperExplorer").size(24.0);
                    ui.label(header);
                    ui.add(egui::TextEdit::singleline(&mut self.curr_dir_text).interactive(false));

                    if self.is_indexing {
                        ui.label(egui::RichText::new(&self.status_msg).color(egui::Color32::YELLOW));
                    } else if self.status_msg.contains("Indexed") && self.status_msg.contains("files in") {
                        ui.label(egui::RichText::new(&self.status_msg).color(egui::Color32::GREEN));
                    } else if self.status_msg.contains("error") {
                        ui.label(egui::RichText::new(&self.status_msg).color(egui::Color32::RED));
                    } else {
                        ui.label(&self.status_msg);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Search:");
                    let search_enabled = !self.is_indexing && self.indexed_files > 0;

                    let search_response = ui.add_enabled(
                        search_enabled,
                        egui::TextEdit::singleline(&mut self.search)
                            .hint_text("Type and press enter to search...")
                    );

                    if search_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.perform_search(ctx);
                    }

                    if search_enabled && !self.search.is_empty() {
                        if ui.button("Clear").clicked() {
                            self.search.clear();
                            self.search_results.clear();
                            self.show_search_results = false;
                        }
                    }
                });
                self.show_disks(ui, ctx);

                if self.show_search_results {
                    ui.separator();
                    ui.label(format!("Found {} results:", self.search_results.len()));

                    egui::ScrollArea::vertical()
                        .id_salt("search_results")
                        .max_height(200.0)
                        .show(ui, |ui| {
                            for result in &self.search_results {
                                let (icon, size_text) = if result.is_dir {
                                    (egui::Image::new(egui::include_image!("icons/folder.png")), "DIR".to_string())
                                } else {
                                    let icon = icons::get_file_icon(&result.name);
                                    (icon, crate::utils::format_file_size(result.size))
                                };

                                ui.horizontal(|ui| {
                                    let button = egui::Button::image_and_text(icon, format!("{} ({})", result.name, size_text));
                                    if ui.add(button).clicked() {
                                        if result.is_dir {
                                            self.curr_dir = Some(result.path.clone());
                                            self.curr_dir_text = result.path.to_string_lossy().to_string();
                                            self.dir_changed = true;
                                            self.is_root = false;
                                            self.show_search_results = false;
                                        } else {
                                            let _ = opener::open(&result.path);
                                        }
                                    }
                                    ui.label(format!("Path: {}", result.path.parent().unwrap_or(&result.path).display()));
                                });
                            }
                        });
                }

                if let Some(_) = &self.curr_dir {
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .id_salt("file_browser")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            if !self.show_search_results {
                                self.show_files(ui, ctx);
                            }
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