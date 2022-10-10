use eframe::{egui, App};

#[derive(Default)]
struct RTunes {}

impl App for RTunes {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hello world");
        });
    }
}

impl RTunes {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "RTunes",
        native_options,
        Box::new(|cc| Box::new(RTunes::new(cc))),
    );
}
