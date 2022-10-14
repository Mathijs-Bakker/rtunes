use eframe::{
    egui::{self, ImageButton},
    epaint::Vec2,
    App,
};
use egui_extras::{RetainedImage, Size, StripBuilder};

// #[derive(Default)]
struct RTunes {
    toolbar_icon: ToolBarIcons,
    player_state: PlayerState,
}

impl RTunes {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            toolbar_icon: ToolBarIcons::new(),
            player_state: PlayerState::new(),
        }
    }
}

impl App for RTunes {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar_panel").show(ctx, |ui| {
            let n_toolbar_icons = 5.;
            let horizontal_offset = 15.;
            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::exact(
                    (self.toolbar_icon.size.x + horizontal_offset) * n_toolbar_icons,
                ))
                .size(Size::remainder())
                .horizontal(|mut strip| {
                    strip.empty();
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 5).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                if ui
                                    .add(ImageButton::new(
                                        self.toolbar_icon.open_song.texture_id(ctx),
                                        self.toolbar_icon.size,
                                    ))
                                    .clicked()
                                {
                                    // *label_button_clicked = "clicked".to_string();
                                };
                            });
                            strip.cell(|ui| {
                                ui.add(ImageButton::new(
                                    self.toolbar_icon.play_previous.texture_id(ctx),
                                    self.toolbar_icon.size,
                                ));
                            });

                            // Play button:
                            strip.cell(|ui| {
                                if !self.player_state.is_playing {
                                    if ui
                                        .add(ImageButton::new(
                                            self.toolbar_icon.play.texture_id(ctx),
                                            self.toolbar_icon.size,
                                        ))
                                        .clicked()
                                    {
                                        self.player_state.is_playing = true;
                                    }
                                } else {
                                    if ui
                                        .add(ImageButton::new(
                                            self.toolbar_icon.pauze.texture_id(ctx),
                                            self.toolbar_icon.size,
                                        ))
                                        .clicked()
                                    {
                                        self.player_state.is_playing = false;
                                    }
                                }
                            });
                            strip.cell(|ui| {
                                ui.add(ImageButton::new(
                                    self.toolbar_icon.play_next.texture_id(ctx),
                                    self.toolbar_icon.size,
                                ));
                            });
                            strip.cell(|ui| {
                                ui.add(ImageButton::new(
                                    self.toolbar_icon.stop.texture_id(ctx),
                                    self.toolbar_icon.size,
                                ));
                            });
                        });
                    });
                    strip.empty();
                });
        });
    }
}

struct PlayerState {
    is_playing: bool,
}

impl PlayerState {
    fn new() -> Self {
        Self { is_playing: false }
    }
}

struct ToolBarIcons {
    size: Vec2,
    open_song: RetainedImage,
    pauze: RetainedImage,
    play: RetainedImage,
    play_next: RetainedImage,
    play_previous: RetainedImage,
    stop: RetainedImage,
}

impl ToolBarIcons {
    fn new() -> Self {
        Self {
            size: Vec2 { x: 25., y: 25. },
            open_song: RetainedImage::from_image_bytes(
                "add-button.png",
                include_bytes!("../assets/add-button.png"),
            )
            .unwrap(),
            pauze: RetainedImage::from_image_bytes(
                "pauze-button.png",
                include_bytes!("../assets/pauze-button.png"),
            )
            .unwrap(),
            play: RetainedImage::from_image_bytes(
                "play-button.png",
                include_bytes!("../assets/play-button.png"),
            )
            .unwrap(),
            play_next: RetainedImage::from_image_bytes(
                "next-button.png",
                include_bytes!("../assets/next-button.png"),
            )
            .unwrap(),
            play_previous: RetainedImage::from_image_bytes(
                "previous-button.png",
                include_bytes!("../assets/previous-button.png"),
            )
            .unwrap(),
            stop: RetainedImage::from_image_bytes(
                "stop-button.png",
                include_bytes!("../assets/stop-button.png"),
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
