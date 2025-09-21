#![windows_subsystem = "windows"]
use tokio;
mod ui;
mod filesystem;
mod search;
mod icons;
mod ctxmenus;
mod utils;

#[tokio::main]
async fn main() {
    let nativeopt = eframe::NativeOptions::default();
    let search_engine = search::FileSearcher::new();
    eframe::run_native(
        "HyperExplorer",
        nativeopt,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ui::HyperExplorer::new(cc, search_engine)))
        })
    ).expect("An error occured while running HyperExplorer");
}
