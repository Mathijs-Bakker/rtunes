use eframe::{
    egui::{self, ImageButton},
    epaint::Vec2,
    App,
};
use egui_extras::RetainedImage;

// #[derive(Default)]
struct RTunes {
    play_button: PlayImage,
}

impl App for RTunes {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let size = Vec2 { x: 50., y: 50. };
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hello world");
            // ui.add(egui::ImageButton::new(self.image.texture_id(ctx)));
            ui.add(ImageButton::new(
                self.play_button.image.texture_id(ctx),
                size,
            ))
        });
    }
}

impl RTunes {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            play_button: PlayImage::new(),
        }
        // Self::default()
    }
}

struct PlayImage {
    image: RetainedImage,
}

impl PlayImage {
    fn new() -> Self {
        Self {
            image: RetainedImage::from_image_bytes(
                "play-button.png",
                include_bytes!("../assets/play-button.png"),
            )
            .unwrap(),
        }
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(500., 500.)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "RTunes",
        native_options,
        Box::new(|cc| Box::new(RTunes::new(cc))),
    );
}
