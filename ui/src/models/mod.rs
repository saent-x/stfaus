use serde::{Deserialize, Serialize};



#[derive(Debug, Hash, Default, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq)]
pub(crate) struct Song {
    pub uuid: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub cover_art: String,
    pub preview_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct UserPromptRequest {
    pub prompt: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettingsUpdate {
    pub user_name: String,
    pub music_era: String,
    pub music_genre: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MusicEraGenreList {
    pub music_eras: Vec<String>,
    pub music_genres: Vec<String>
}