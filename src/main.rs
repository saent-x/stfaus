mod prelude;
mod site_router;
mod pages;
mod components;
mod libs;
mod models;
mod hub;
mod utils;

use dioxus::prelude::*;

use libs::db::get_playlist;
use prelude::{get_db, AppState, FAVICON, MAIN_CSS, TAILWIND_CSS};


fn main(){
    dioxus::launch(App);    
}

#[component]
fn App() -> Element {
    let playlist = use_resource(move || async move {
        let mut db = get_db().lock().await;
        let saved_playlist = get_playlist(&mut db).unwrap_or_default();

        saved_playlist
    }).suspend()?;
        
    use_context_provider(|| Signal::new(AppState{
        searching: false,
        current_playlist: playlist(),
    }));
    
    rsx! {
        meta {
            name: "viewport",
            content: "width=device-width, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no viewport-fit=cover"
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<site_router::Route> {}
    }
}