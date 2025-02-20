use crate::models::{AppError, LLMResult, SearchAgent, Song};
use async_trait::async_trait;
use rspotify::{
    model::{SearchResult, SearchType},
    prelude::*,
    ClientCredsSpotify, Credentials,
};

pub struct SpotifySearchResult {}
pub struct SpotifyAgent {
    client: ClientCredsSpotify,
}

impl SpotifyAgent {
    pub async fn init() -> Result<Self, AppError> {
        let creds = Credentials::from_env().expect("Missing SPOTIFY_CLIENT_ID or SPOTIFY_CLIENT_SECRET");
        // let oauth = OAuth::default();
        let client = ClientCredsSpotify::new(creds);
        client.request_token().await.map_err(|err| AppError::SearchAgentError(err.to_string()))?;

        Ok(Self { client })
    }
}

#[async_trait]
impl SearchAgent for SpotifyAgent {
    async fn search(&self, search_items: &[LLMResult]) -> Result<Vec<Song>, AppError> {
        let mut results: Vec<Song> = vec![];

        for llmresult in search_items {
            let query = format!("track:{} artist:{} year:{}", llmresult.title.clone().unwrap_or_default(), llmresult.artist.clone().unwrap_or_default(), llmresult.year.clone().unwrap_or_default());

            let result = self.client.search(&query, SearchType::Track, None, None, Some(1), None).await.map_err(|err| AppError::SearchAgentError(err.to_string()))?;

            if let SearchResult::Tracks(tracks) = result {
                if let Some(track) = tracks.items.first() {
                    let song = Song {
                        uuid: track.id.as_ref().unwrap().to_string(), //TODO: better handling of option
                        title: track.name.clone(),
                        artist: track.artists[0].name.clone(),
                        album: track.album.name.clone(), 
                        cover_art: track.album.images[0].url.clone(),
                    };

                    results.push(song);
                }
            }
        }

        Ok(results)
    }
}
