use std::sync::Arc;

use crate::{
    get_app_engine,
    models::{MusicEra, MusicGenre, Song},
    prelude::AppError,
};
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
};
use polodb_core::Database;

use super::{
    db::{AppData, get_playlist, save_playlist},
    schema::{AppSettingsUpdate, UserPromptRequest},
};

#[axum::debug_handler]
pub async fn search_playlist(
    State(db): State<Arc<Database>>,
    payload: Result<Json<UserPromptRequest>, JsonRejection>,
) -> Result<Json<Vec<Song>>, AppError> {
    let prompt_request = payload?;
    let app_data = super::db::get_app_settings(&db)?;
    let app = get_app_engine().await;

    let response = app.lock().await
        .ask(
            &prompt_request.prompt,
            app_data
                .music_genre
                .parse::<MusicGenre>()
                .unwrap_or(MusicGenre::Any),
            app_data
                .music_era
                .parse::<MusicEra>()
                .unwrap_or(MusicEra::Any),
        ).await?;

    save_playlist(&db, &response)?;

    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn get_saved_playlist(State(db): State<Arc<Database>>) -> Result<Json<Vec<Song>>, AppError> {
    let playlist = get_playlist(&db)?;

    Ok(Json(playlist))
}

#[axum::debug_handler]
pub async fn get_app_settings(State(db): State<Arc<Database>>) -> Result<Json<AppData>, AppError> {
    let data = super::db::get_app_settings(&db)?;

    Ok(Json(data))
}

#[axum::debug_handler]
pub async fn update_app_data(
    State(db): State<Arc<Database>>,
    payload: Result<Json<AppSettingsUpdate>, JsonRejection>,
) -> Result<(), AppError> {
    let payload = payload?;
    super::db::save_app_settings(
        &db,
        payload.user_name.clone(),
        payload.music_era.clone(),
        payload.music_genre.clone(),
    )?;

    Ok(())
}
