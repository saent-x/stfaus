use dotenvy::dotenv;
use llm::{hub::engine::AppEngine, models::{Config, MusicEra}};


// const GEMINI_MODEL: &str = "gemini-2.0-flash";
const OPENAI_MODEL: &str = "gpt-4o-mini";
    
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    
    let config = Config::build(OPENAI_MODEL, MusicEra::Modern, "".to_string());
    let app = AppEngine::init_llm(&config);
    
    let expression = "today is a bring and beautiful day"; // from input box
    let question = format!("generate 8 data in json array having [artist, title, album, year, genre] for {} songs that reflect the feeling of the statement. I only want the raw json not md format and nothing else in the respose so its parsable'{}'", config.music_era.to_string(), expression);
    
    let search_res = app.ask(&question).await?;
    

    println!("Translation:\n{:?}", search_res.results[0]);
    Ok(())
}
