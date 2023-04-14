use crate::wishlist::*;
use eframe::egui;
use egui::{FontFamily::*, FontId, TextStyle::*};
use std::fs;

pub struct MyApp {
    steam_id: String,
    games: Vec<GameInfo>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            steam_id: String::from("76561198428243990"),
            games: vec![GameInfo::with_error_msg("Change your Steam ID to your own")],
        }
    }
}

impl MyApp {
    pub fn new() -> Self {
        match fs::read_to_string("./assets/steam_id.txt") {
            Ok(steam_id) => {
                match download_wishlist(&steam_id) {
                    Ok(wishlist) => Self {
                        steam_id: steam_id,
                        games: wishlist,
                    },
                    Err(_) => Self {
                        steam_id: steam_id,
                        games: Self::get_error_msg()
                    }
                }
            },
            Err(_) => {
                Self::default()
            }
        }
    }
    fn download_games(&mut self) {
        match download_wishlist(&self.steam_id) {
            Ok(games) => {
                self.games = games;
            }
            Err(_) => {
                self.show_download_error();
            }
        }
    }
    fn save_steam_id(&self) {
        fs::write("./assets/steam_id.txt", &self.steam_id).expect("Unable to write file");
    }
    fn show_download_error(&mut self) {
        self.games = Self::get_error_msg();
    }
    fn get_error_msg() -> Vec<GameInfo> {
        vec![
            GameInfo::with_error_msg("Can't download wishlist"),
            GameInfo::with_error_msg("Check your internet connection"),
            GameInfo::with_error_msg("Check your Steam ID is correct"),
            GameInfo::with_error_msg("Make sure your account is public"),
        ]
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            change_font_size(ctx);

            ui.heading("😎 Steam Wishlist Spy 😎");
            ui.label("");
            ui.horizontal(|ui| {
                let name_label = ui.label("🕵 Your Steam ID: ");
                ui.text_edit_singleline(&mut self.steam_id)
                    .labelled_by(name_label.id);
                if ui.button("💾 | ↺").clicked() {
                    self.save_steam_id();
                    self.download_games();
                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("");
                for game in &self.games {
                    ui.horizontal(|ui| {
                        ui.label(&game.name);
                        if game.subs.len() > 0 {
                            let mut price = game.subs[0].price.to_string();
                            price.truncate(price.len() - 2);
                            ui.label(format!(
                                "- {}₴, {}%",
                                price,
                                game.subs[0].discount_pct.to_string()
                            ));
                            if game.subs[0].discount_pct > 0 {
                                ui.label(" 📉");
                            }
                        } else {
                            ui.label(" 😓");
                        }
                    });
                }
            });
        });
    }
}

fn change_font_size(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (Heading, FontId::new(27.0, Proportional)),
        (Body, FontId::new(20.0, Proportional)),
        (Button, FontId::new(20.0, Proportional)),
        (Small, FontId::new(15.0, Proportional)),
    ]
    .into();

    ctx.set_style(style);
}
