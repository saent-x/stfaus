use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use core::fmt;


#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct SearchResult {
    pub results: Vec<LLMResult>
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct LLMResult {
    pub artist: Option<String>,
    pub title:  Option<String>,
    pub album:  Option<String>,
    pub year:   Option<u32>,
    pub genre:  Option<String>,
}

#[derive(Debug)]
pub struct Song {
   pub title: String,
   pub artist: String,
   pub cover_art: String,
}

pub enum MusicEra {
    Modern,
    Contemporary,
    Early2000,
    Mid2000,
    N90sEra,
    T80sEra,
    Oldies
}

impl fmt::Display for MusicEra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            MusicEra::Modern => "Modern",
            MusicEra::Contemporary => "Contemporary",
            MusicEra::Early2000 => "Early 2000",
            MusicEra::Mid2000 => "Mid 2000",
            MusicEra::N90sEra => "90s Era",
            MusicEra::T80sEra => "80s Era",
            MusicEra::Oldies => "Oldies"
        })
    }
}

pub struct Config<'a> {
    pub model: &'a str,
    pub music_era: MusicEra,
    pub context: String,
}

impl<'a> Config<'a> {
    pub fn build(model: &'a str, music_era: MusicEra, context: String) -> Self {
        Config { model, music_era, context }
    }
}