mod prelude;
mod site_router;
mod pages;
mod components;

mod models;

mod services;
mod error;

use dioxus::prelude::*;
use prelude::{AppState, FAVICON, STYLE_CSS, TAILWIND_CSS};



fn main(){
    dioxus::launch(App);
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