#![windows_subsystem = "windows"]
use tokio;
mod ui;
mod filesystem;

#[tokio::main]
async fn main() {
    let nativeopt = eframe::NativeOptions::default();
    eframe::run_native(
        "HyperExplorer",
        nativeopt,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ui::HyperExplorer::new(cc)))
        })
    ).expect("An error occured while running HyperExplorer");
}
