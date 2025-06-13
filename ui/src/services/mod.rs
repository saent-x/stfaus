
use dioxus::logger::tracing::info;

use crate::models::{AppSettingsUpdate, MusicEraGenreList, Song, UserPromptRequest};

const BASE_URL: &str = "http://127.0.0.1:3000";

pub(crate) async fn search_playlist(user_prompt: &UserPromptRequest) -> dioxus::Result<Vec<Song>>{    
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/search", BASE_URL))
        .json(&user_prompt)
        .send()
        .await?
        .json::<Vec<Song>>()
        .await?;
    
    Ok(res)
}

pub(crate) async fn get_playlist() -> dioxus::Result<Vec<Song>>{
    let client = reqwest::Client::new();
    let res = client.get(format!("{}/playlist", BASE_URL))
        .send()
        .await?
        .json::<Vec<Song>>()
        .await?;
        
    Ok(res)
}

pub(crate) async fn get_music_era_genre_list() -> dioxus::Result<MusicEraGenreList>{
    let client = reqwest::Client::new();
    let res = client.get(format!("{}/era_genre_list", BASE_URL))
        .send()
        .await?
        .json::<MusicEraGenreList>()
        .await?;
    
    Ok(res)
}

pub(crate) async fn get_settings() -> dioxus::Result<AppSettingsUpdate>{
    let client = reqwest::Client::new();
    let res = client.get(format!("{}/settings", BASE_URL))
        .send()
        .await?
        .json::<AppSettingsUpdate>()
        .await?;
    
    Ok(res)
}

pub(crate) async fn update_settings(update: AppSettingsUpdate) -> dioxus::Result<bool>{
    let client = reqwest::Client::new();
    let res = client.post(format!("{}/update_settings", BASE_URL))
        .json(&update)
        .send()
        .await?
        .json::<bool>()
        .await?;
    
    Ok(res)
}