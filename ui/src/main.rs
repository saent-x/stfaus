mod prelude;
mod site_router;
mod pages;
mod components;

mod libs;

mod models;
mod hub;
mod utils;



#[cfg(feature = "server")]
use {
    crate::hub::engine::AppEngine,
    diesel::r2d2::{ConnectionManager, Pool},
    diesel::SqliteConnection,
    tokio::sync::Mutex,
    std::sync::Arc,
};


use dioxus::{cli_config, prelude::*};
use prelude::{AppState, FAVICON, STYLE_CSS, TAILWIND_CSS};



fn main(){
    #[cfg(feature = "server")]
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                launch_server().await
            });
        
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[cfg(feature = "server")]
#[derive(Clone)]
struct ServerAppState<'a> {
    app: Arc<Mutex<AppEngine<'a>>>,
    db: Pool<ConnectionManager<SqliteConnection>>,
    app_state: AppState
}

#[cfg(feature = "server")]
async fn launch_server() {
    use std::sync::Arc;

    use axum;
    use axum::Router;
    use libs::db::create_db;
    
    let db_pool = create_db();
    let app_state = AppState{
        searching: false,
        current_playlist: get_playlist(&mut db_pool.get().unwrap()).unwrap_or_default(),
    };
    
    let server_app_state = ServerAppState{
        app: Arc::new(Mutex::new(setup_app().await)),
        db: db_pool,
        app_state
    };
    
    
    
    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = cli_config::fullstack_address_or_localhost();

    // Build a custom axum router
    let router = Router::new()
        .serve_dioxus_application(ServeConfig::new().unwrap(), App)
        .with_state(Arc::new(server_app_state))
        .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[cfg(feature = "server")]
async fn setup_app<'a>() -> AppEngine<'a> {
    use models::Config;
    use crate::hub::spotify::SpotifyAgent;
    use crate::hub::engine::AppEngine;
    
    pub const AI_MODEL: &str = "o3";

    let config = Config::build(
        AI_MODEL, 
        "".to_string(), 
        8
    );
    
    let spotify_agent = SpotifyAgent::init().await.expect("failed to initialize spotify");
    
    AppEngine::init(config, Box::new(spotify_agent))
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppState{
        searching: false,
        current_playlist: vec![],
    }));
    
    rsx! {
        meta {
            name: "viewport",
            content: "width=device-width, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no viewport-fit=cover",
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: STYLE_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<site_router::Route> {}
    }
}