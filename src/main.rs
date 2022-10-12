use eframe::{
    egui::{self, ImageButton},
    epaint::Vec2,
    App,
};
use egui_extras::RetainedImage;

// #[derive(Default)]
struct RTunes {
    toolbar_icon: ToolBarIcons,
}

impl App for RTunes {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::Grid::new("tool_bar")
                .num_columns(3)
                .min_col_width(140.)
                .show(ui, |ui| {
                    ui.label("");
                    ui.horizontal(|ui| {
                        ui.add(ImageButton::new(
                            self.toolbar_icon.open_song.texture_id(ctx),
                            self.toolbar_icon.size,
                        ));
                        ui.add(ImageButton::new(
                            self.toolbar_icon.play_previous.texture_id(ctx),
                            self.toolbar_icon.size,
                        ));
                        ui.add(ImageButton::new(
                            self.toolbar_icon.play.texture_id(ctx),
                            self.toolbar_icon.size,
                        ));
                        ui.add(ImageButton::new(
                            self.toolbar_icon.play_next.texture_id(ctx),
                            self.toolbar_icon.size,
                        ));
                        ui.add(ImageButton::new(
                            self.toolbar_icon.pauze.texture_id(ctx),
                            self.toolbar_icon.size,
                        ));
                    });
                });

            ui.label("");
            ui.end_row();
        });
    }
}

impl RTunes {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            toolbar_icon: ToolBarIcons::new(),
        }
    }
}

struct ToolBarIcons {
    size: Vec2,
    open_song: RetainedImage,
    pauze: RetainedImage,
    play: RetainedImage,
    play_next: RetainedImage,
    play_previous: RetainedImage,
}

impl ToolBarIcons {
    fn new() -> Self {
        Self {
            size: Vec2 { x: 25., y: 25. },
            open_song: RetainedImage::from_image_bytes(
                "play-button.png",
                include_bytes!("../assets/play-button.png"),
            )
            .unwrap(),
            pauze: RetainedImage::from_image_bytes(
                "play-button.png",
                include_bytes!("../assets/play-button.png"),
            )
            .unwrap(),
            play: RetainedImage::from_image_bytes(
                "play-button.png",
                include_bytes!("../assets/play-button.png"),
            )
            .unwrap(),
            play_next: RetainedImage::from_image_bytes(
                "play-button.png",
                include_bytes!("../assets/play-button.png"),
            )
            .unwrap(),
            play_previous: RetainedImage::from_image_bytes(
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
