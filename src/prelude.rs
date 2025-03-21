use std::sync::OnceLock;

use diesel::SqliteConnection;
use dioxus::prelude::*;
use tokio::sync::Mutex;

use crate::{hub::{engine::AppEngine, spotify::SpotifyAgent}, libs::db::{create_db, get_app_settings, AppData}, models::{AppError, Config, MusicEra, MusicGenre, Song}};

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const T3: Asset = asset!("/assets/ti3.jpg");
pub const SEARCHING_GIF: Asset = asset!("/assets/searching.gif");


pub const OPENAI_MODEL: &str = "gpt-4o-mini";

#[derive(Clone)]
pub struct AppState {
    pub searching: bool,
    pub current_playlist: Vec<Song>,
}

static APP_ENGINE: OnceLock<Mutex<AppEngine<'static>>> = OnceLock::new();
static DB: OnceLock<Mutex<SqliteConnection>> = OnceLock::new();


pub fn get_app_engine() -> &'static Mutex<AppEngine<'static>> {
    APP_ENGINE.get_or_init(|| {
        let config = Config::build(
            OPENAI_MODEL, 
            "".to_string(), 
            8
        );
        let spotify_agent = {
            if let Ok(handle) = tokio::runtime::Handle::try_current() {
                tokio::task::block_in_place(|| {
                    handle.block_on(SpotifyAgent::init())
                        .expect("Failed to initialize SpotifyAgent")
                })
            } else {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to build temporary runtime");

                rt.block_on(SpotifyAgent::init())
                    .expect("Failed to initialize SpotifyAgent")
            }
        };

        dbg!("In line");
        Mutex::new(AppEngine::init(config, Box::new(spotify_agent)))
    })
}

pub fn get_db() -> &'static Mutex<SqliteConnection> {
    DB.get_or_init(|| {
        Mutex::new(create_db())
    })
}