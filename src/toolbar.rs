use eframe::epaint::Vec2;
use egui_extras::RetainedImage;

pub(crate) struct ToolBarIcons {
    pub(crate) size: Vec2,
    pub(crate) open_song: RetainedImage,
    pub(crate) pauze: RetainedImage,
    pub(crate) play: RetainedImage,
    pub(crate) play_next: RetainedImage,
    pub(crate) play_previous: RetainedImage,
    pub(crate) stop: RetainedImage,
}

impl ToolBarIcons {
    pub(crate) fn new() -> Self {
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
