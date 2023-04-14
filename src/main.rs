// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod steam_wishlist_spy_app;
mod wishlist;

use eframe::egui;
use steam_wishlist_spy_app::MyApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        resizable: true,
        icon_data: Some(load_icon("./assets/pic.png")),
        initial_window_size: Some(egui::vec2(650.0, 650.0)),
        min_window_size: Some(egui::vec2(600.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Steam Wishlist Spy",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
