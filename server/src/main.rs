use std::{str::FromStr, sync::Arc};
use libs::handlers::{get_app_settings, get_era_n_genre_list, get_saved_playlist, search_playlist, update_settings};
use tokio::sync::OnceCell;

use axum::{http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderName, HeaderValue, Method}, routing::{get, post}, Router};
use hub::{engine::AppEngine, spotify::SpotifyAgent};
use models::Config;
use polodb_core::Database;
use tokio::sync::Mutex;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod hub;
mod libs;
mod models;
mod prelude;
mod utils;

static APP_ENGINE: OnceCell<Mutex<AppEngine<'static>>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!(
                "{}=debug,tower_http=debug,axum::rejection=trace",
                env!("CARGO_CRATE_NAME")
            )
            .into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Database::open_path("stfaus.db").expect("database not found");

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(["http://127.0.0.1:8080".parse::<HeaderValue>().unwrap()])
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, HeaderName::from_str("API_KEY").unwrap()]);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    println!("[INFO] server listening @ {}", addr);

    axum::serve(listener, load_router(Arc::new(db), cors).await)
        .await
        .unwrap(); // TODO: setup cors later
}

async fn get_app_engine() -> &'static Mutex<AppEngine<'static>> {
    APP_ENGINE.get_or_init(|| async {
        let config = Config::build(
            "gpt-4.1-nano",
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

async fn load_router(app_state: Arc<Database>, cors: CorsLayer) -> Router {
    Router::new()
        .route("/search", post(search_playlist))
        .route("/playlist", get(get_saved_playlist))
        .route("/settings", get(get_app_settings))
        .route("/update_settings", post(update_settings))
        .route("/era_genre_list", get(get_era_n_genre_list))
        .with_state(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
