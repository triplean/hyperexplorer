#![windows_subsystem = "windows"]
use tokio;
mod ui;
mod indexing;
mod mods;
mod dialogs;

#[tokio::main]
async fn main() {
    let nativeopt = eframe::NativeOptions::default();
    eframe::run_native(
        "HyperExplorer",
        nativeopt,
        Box::new(|cc| Ok(Box::new(ui::HyperExplorer::new(cc))))
    ).expect("An error occured while running HyperExplorer");
}
