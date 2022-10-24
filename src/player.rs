pub(crate) struct PlayerState {
    pub(crate) is_playing: bool,
}

impl PlayerState {
    pub(crate) fn new() -> Self {
        Self { is_playing: false }
    }
}

use crate::toolbar;
use eframe::egui;
use eframe::egui::ImageButton;
use eframe::App;
use egui_extras::Size;
use egui_extras::StripBuilder;

// #[derive(Default)]
pub(crate) struct Player {
    pub(crate) toolbar_icon: toolbar::ToolBarIcons,
    pub(crate) player_state: PlayerState,
}

impl App for Player {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.toolbar(ctx);
    }
}

impl Player {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            toolbar_icon: toolbar::ToolBarIcons::new(),
            player_state: PlayerState::new(),
        }
    }

    pub(crate) fn toolbar(&mut self, ctx: &egui::Context) {
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
                            self.play_button(&mut strip, ctx);
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

    fn play_button(&mut self, strip: &mut egui_extras::Strip, ctx: &egui::Context) {
        strip.cell(|ui| {
            if self.player_state.is_playing {
                if ui
                    .add(ImageButton::new(
                        self.toolbar_icon.pauze.texture_id(ctx),
                        self.toolbar_icon.size,
                    ))
                    .clicked()
                {
                    self.player_state.is_playing = false;
                }
            } else if ui
                .add(ImageButton::new(
                    self.toolbar_icon.play.texture_id(ctx),
                    self.toolbar_icon.size,
                ))
                .clicked()
            {
                self.player_state.is_playing = true;
            }
        });
    }
}
