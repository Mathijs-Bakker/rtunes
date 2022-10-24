#![warn(clippy::all, clippy::pedantic)]

use eframe::epaint::Vec2;

mod player;

mod toolbar;

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(500., 500.)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "RTunes",
        native_options,
        Box::new(|cc| Box::new(player::Player::new(cc))),
    );
}
