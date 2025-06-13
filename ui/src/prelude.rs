use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::Song;


pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const STYLE_CSS: Asset = asset!("/assets/style.scss");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const SEARCHING_GIF: Asset = asset!("/assets/searching.gif");

pub static CURRENT_PLAYLIST: GlobalSignal<Vec<Song>> = Signal::global(|| vec![]);
pub static CURRENT_PROMPT: GlobalSignal<String> = Signal::global(|| "I'm feeling lucky".to_string());

#[derive(Clone, Serialize, Deserialize)]
pub struct AppState {
    pub searching: bool,
    pub current_playlist: Vec<Song>,
}