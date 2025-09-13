use crate::ui;

pub fn error_dialog(title: String, message: String) {
    let nativeopt = eframe::NativeOptions::default();
    let dial = eframe::run_native(
        "HyperExplorer",
        nativeopt,
        Box::new(|cc| Ok(Box::new(Dialogs::new(message.clone()))))
    );

    match dial {
        Ok(_) => {},
        Err(e) => {
            println!("Looks like you have a really bad luck. 2 whole errors in a row");
            println!("Original error: {}", message);
            println!("Dialog error: {}", e);
        }
    }
}


struct Dialogs {
    message: String,
}

impl Dialogs {
    pub fn new( message: String) -> Dialogs {
        Dialogs {
            message
        }
    }
}

impl eframe::App for Dialogs {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.message.clone());
        });
    }
}