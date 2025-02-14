use crate::models::{Config, LLMResult, SearchResult};
use rig::{agent::Agent, completion::Prompt, providers::openai::{self, CompletionModel}};

pub struct AppEngine {
    pub agent: Agent<CompletionModel>
}

impl AppEngine {
    pub fn init_llm(config: &Config) -> AppEngine {
        let client = openai::Client::from_env();
        let agent = client.agent(config.model).build();
        
        AppEngine { agent }
    }
    
    pub async fn ask(&self, question: &str) -> Result<SearchResult, Box<dyn std::error::Error>>{
        let response = self.agent.prompt(question).await?;
        
        // let gpt_extractor = client.extractor::<SearchResult>(config.model).build();
        // let search_results = gpt_extractor.extract(&response).await?;
        let results: Vec<LLMResult> = serde_json::from_str(&response).expect("Err: Failed to parse json");
        
        Ok(SearchResult { results })
    }
}
