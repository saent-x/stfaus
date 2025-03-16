use dioxus::prelude::*;

use crate::site_router;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { id: "nav",
            div { id: "body", Outlet::<site_router::Route> {} }
        }
    }
}
