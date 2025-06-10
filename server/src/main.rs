use std::sync::Arc;
use libs::handlers::{get_app_settings, get_saved_playlist, search_playlist, update_app_data};
use tokio::sync::OnceCell;

use axum::{routing::{get, post}, Router};
use hub::{engine::AppEngine, spotify::SpotifyAgent};
use models::Config;
use polodb_core::Database;
use tokio::sync::Mutex;

mod hub;
mod libs;
mod models;
mod prelude;
mod utils;

static APP_ENGINE: OnceCell<Mutex<AppEngine<'static>>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = Database::open_path("stfaus.db").expect("database not found");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    println!("[INFO] server listening @ {}", addr);

    axum::serve(listener, load_router(Arc::new(db)).await)
        .await
        .unwrap(); // TODO: setup cors later
}

async fn get_app_engine() -> &'static Mutex<AppEngine<'static>> {
    APP_ENGINE.get_or_init(|| async {
        let config = Config::build(
            "o3",
            "
                You are an AI system that generates a playlist based on the user's
                prompt and you return the response as a raw json and not md format
                with nothing else in the response so its parsable.
                "
            .to_string(),
            10,
        );

        let spotify_agent = SpotifyAgent::init().await.unwrap();
        let app = AppEngine::init(config, Box::new(spotify_agent));

        Mutex::new(app)
    }).await
}

async fn load_router(app_state: Arc<Database>) -> Router {
    Router::new()
        .route("/search", post(search_playlist))
        .route("/playlist", get(get_saved_playlist))
        .route("/settings", get(get_app_settings))
        .route("/update_settings", post(update_app_data))
        .with_state(app_state)
}
