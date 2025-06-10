use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptRequest {
    pub prompt: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettingsUpdate {
    pub uuid: String,
    pub user_name: String,
    pub music_era: String,
    pub music_genre: String,
}