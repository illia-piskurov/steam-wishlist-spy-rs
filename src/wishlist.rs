use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInfo {
    pub name: String,
    pub subs: Vec<Subs>,
}

impl GameInfo {
    pub fn with_error_msg(str: &str) -> Self {
        Self {
            name: String::from(str),
            subs: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subs {
    pub price: u64,
    pub discount_pct: u64,
}

pub fn download_wishlist(steam_id: &str) -> Result<Vec<GameInfo>, Box<dyn std::error::Error>> {
    let endpoint = format_endpoint(steam_id);
    let resp = reqwest::blocking::get(endpoint)?.json::<HashMap<String, GameInfo>>()?;

    let result: Vec<GameInfo> = resp.into_iter().map(|(_id, game_info)| game_info).collect();

    Ok(result)
}

pub fn format_endpoint(steam_id: &str) -> String {
    format!(
        "https://store.steampowered.com/wishlist/profiles/{}/wishlistdata/",
        steam_id
    )
}
