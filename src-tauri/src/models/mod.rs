use async_trait::async_trait;
use core::fmt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait SearchAgent {
    async fn search(&self, search_items: &[LLMResult]) -> Result<Vec<Song>, AppError>;
}

#[derive(Debug)]
pub enum AppError {
    SearchAgentError(String),
    AppEngineError(String),
}

#[derive(Debug, Deserialize, Default, JsonSchema, Serialize)]
pub struct SearchResult {
    pub results: Vec<LLMResult>,
}

#[derive(Debug, Deserialize, Clone, JsonSchema, PartialEq, PartialOrd, Serialize)]
pub struct LLMResult {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub year: Option<u32>,
    pub genre: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd, Eq)]
pub struct Song {
    pub uuid: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub cover_art: String,
}

pub enum MusicEra {
    Modern,
    Contemporary,
    Early2000,
    Mid2000,
    N90sEra,
    T80sEra,
    Oldies,
}

pub enum MusicGenre {
    
}

impl fmt::Display for MusicEra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MusicEra::Modern => "Modern",
                MusicEra::Contemporary => "Contemporary",
                MusicEra::Early2000 => "Early 2000",
                MusicEra::Mid2000 => "Mid 2000",
                MusicEra::N90sEra => "90s Era",
                MusicEra::T80sEra => "80s Era",
                MusicEra::Oldies => "Oldies",
            }
        )
    }
}

pub struct Config<'a> {
    pub model: &'a str,
    pub music_era: MusicEra,
    pub context: String,
    pub track_count: u16,
}

impl<'a> Config<'a> {
    pub fn build(model: &'a str, music_era: MusicEra, context: String, track_count: u16) -> Self {
        Config { model, music_era, context, track_count }
    }
}
