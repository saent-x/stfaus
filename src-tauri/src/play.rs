use dotenvy::dotenv;
use stfaus_lib::{
    hub::{engine::AppEngine, spotify::SpotifyAgent},
    models::{AppError, Config, MusicEra},
};

// const GEMINI_MODEL: &str = "gemini-2.0-flash";
const OPENAI_MODEL: &str = "gpt-4o-mini";

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    let config = Config::build(OPENAI_MODEL, MusicEra::Modern, "".to_string(), 8);
    let spotify_agent = Box::new(SpotifyAgent::init().await?);

    let mut app = AppEngine::init(&config, spotify_agent);

    let expression = "today is a bright and beautiful day"; // from input box
    let search_res = app.ask(&expression).await?;

    println!("Spotify Output:\n{}\n", serde_json::to_string_pretty(&search_res).unwrap());

    app.generate_prompt_summary();

    Ok(())
}
