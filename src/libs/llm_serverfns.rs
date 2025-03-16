use crate::{
    models::{MusicEra, MusicGenre, Song},
    prelude::*,
};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

use super::db::{get_app_settings, save_playlist};

//#[server(SearchPlaylistServer)]
pub async fn search_playlist(expression: String) -> Result<Vec<Song>, ServerFnError> {
    let mut app = get_app_engine().lock().await;
    let mut db = get_db().lock().await;

    let app_data = get_app_settings(&mut db);

    let response = app
        .ask(
            &expression,
            app_data
                .music_genre
                .parse::<MusicGenre>()
                .unwrap_or(MusicGenre::Any),
            app_data
                .music_era
                .parse::<MusicEra>()
                .unwrap_or(MusicEra::Any),
        )
        .await
        .map_err(|_| ServerFnError::new("[ERROR] failed to make ask request"))?; // implement to_string for AppError

    let _ = save_playlist(&mut db, &response).expect("failed to save playlist");

    Ok(response)
}

pub async fn play_audio(preview_url: &str) -> Result<(), ServerFnError> {
    Ok(())
}
