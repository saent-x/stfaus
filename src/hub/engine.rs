#![allow(unused)] // thid is a temporary measure to prevent distractions from unused var...
use std::{collections::HashMap, ops::Sub};

use crate::{
    models::{AppError, Config, LLMResult, MusicEra, MusicGenre, SearchAgent, Song},
    utils::{generate_rand_id, remove_duplicates},
};
use rig::{
    agent::Agent,
    completion::Prompt,
    providers::openai::{self, CompletionModel},
};

struct PromptCall {
    count: u32,
    track_count: u16,
}

const MAX_RETRIES: usize = 3;

pub struct AppEngine<'a> {
    pub agent: Agent<CompletionModel>,
    config: Config<'a>,

    search_agent: Box<dyn SearchAgent>,
    prompt_call: HashMap<String, Vec<PromptCall>>,
    prompt_counter: u32,
}

impl<'a> AppEngine<'a> {
    pub fn init(config: Config<'a>, search_agent: Box<dyn SearchAgent>) -> AppEngine<'a> {
        let openai_api_key = env!("OPENAI_API_KEY");
        let client = openai::Client::new(&openai_api_key);
        let agent = client.agent(config.model).build();

        AppEngine { agent, config, search_agent, prompt_call: HashMap::new(), prompt_counter: 0 }
    }

    fn new_prompt_call(&mut self, id: &str, track_count: u16) {
        self.prompt_call.entry(id.to_string())
            .and_modify(|v| v.push(PromptCall { count: self.prompt_counter, track_count })).or_insert(vec![PromptCall { count: self.prompt_counter, track_count }]);
    }

    pub async fn ask(&mut self, expression: &str, music_genre: MusicGenre, music_era: MusicEra) -> Result<Vec<Song>, AppError> {
        let mut results_buffer: Vec<Song> = Vec::new();
        self.send_prompt(
            &generate_rand_id(), 
            0, 
            expression, 
            self.config.track_count,
            &music_genre,
            &music_era,
            &mut results_buffer
        ).await?;

        self.prompt_counter += 0;

        Ok(results_buffer)
    }

    async fn send_prompt(&mut self, id: &str,retry: usize , expression: &str, track_count: u16, music_genre: &MusicGenre, music_era: &MusicEra, results_buffer: &mut Vec<Song>) -> Result<(), AppError> {
        if retry == MAX_RETRIES {
            return Ok(());
        }
        
        self.prompt_counter += 1;
        self.new_prompt_call(id, track_count);

        let question = format!(
            "generate {} data in json array having [artist, title, album, year, genre] for 
            {} songs in {} genre that reflect the feeling of the statement '{}'. I only want the raw json 
            not md format and nothing else in the response so its parsable",
            track_count,
            music_era.to_string(),
            music_genre.to_string(),
            expression
        );

        let response = self.agent.prompt(question).await.map_err(|e| AppError::AppEngineError(e.to_string()))?;
        let results: Vec<LLMResult> = serde_json::from_str(&response).expect("Err: Failed to parse json");

        let spotify_response = self.search_agent.search(&results).await?;
        // println!("GPT Output Count: {} - SPOTIFY Output Count: {}", results.len(), spotify_response.len());
        results_buffer.extend(spotify_response.clone());
        
        let spotify_resp_len = spotify_response.len() - remove_duplicates(results_buffer);
        
        if spotify_resp_len != track_count as usize {
            return Ok(Box::pin(self.send_prompt(id, retry + 1, expression, track_count.sub(spotify_resp_len as u16),music_genre, music_era, results_buffer)).await?);
        }

        Ok(())
    }

    pub fn generate_prompt_summary(&self) {
        self.prompt_call.iter().for_each(|i| {
            println!("\nID: {}", i.0);
            i.1.iter().for_each(|v| println!("iteration number {} for {} tracks", v.count, v.track_count));
        });
    }
}
