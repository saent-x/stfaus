use dioxus::prelude::*;

use crate::components::layout::Layout;
use crate::pages::home::Home;
use crate::pages::settings::Settings;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    
    #[route("/settings")]
    Settings {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
}