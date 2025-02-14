use crate::models::{LLMResult, Song};
use rspotify::{model::SearchType, prelude::*, AuthCodeSpotify, ClientCredsSpotify, Credentials, OAuth};

pub struct SpotifySearchResult {
    
}
pub struct SpotifyAgent {
    client: ClientCredsSpotify
}

impl SpotifyAgent {
    pub fn init() -> Self {
        let creds = Credentials::from_env().expect("Missing SPOTIFY_CLIENT_ID or SPOTIFY_CLIENT_SECRET");
        let oauth = OAuth::default();
        let client = ClientCredsSpotify::new(creds);
        
        Self {client}
    }
    
    pub async fn search_playlist(&self, search_items: Vec<LLMResult>) -> Result<Option<Song>, Box<dyn std::error::Error>>{
        self.client.request_token().await?;
        
        let mut results: Vec<Song> = vec![];
        
        for llmresult in search_items {
            let query = format!("track:{} artist:{} year:{}", 
                llmresult.title.unwrap_or_default(), 
                llmresult.artist.unwrap_or_default(), 
                llmresult.year.unwrap_or_default()
            );
                
            let result = self.client.search(&query, SearchType::Track, None, None, Some(1), None);
            
            let song = match result {
                Ok(tracks) => tracks,
                Err(err) => return Err(err)
            };
            
            if let Some(track) = result.tracks.items.first() {
                let song = Song {
                    title: track.name.clone(),
                    artist: track.artists.first().map(|a| a.name.clone()).unwrap_or_default(),
                    cover_art: track.album.images.first().map(|img| img.url.clone()).unwrap_or_default(),
                };
                results.push(song);
            }
        }
        
        Ok(None)
    }
}