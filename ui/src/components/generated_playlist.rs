use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn GeneratedPlaylist() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    use_server_future(move || {
        async move {
            use crate::ServerAppState;

            let context = server_context();
            let state = context.get::<ServerAppState>().unwrap();

            let mut db = state.db.get().unwrap();
            let playlist = crate::libs::db::get_playlist(&mut db).unwrap();
            app_state.write().current_playlist = playlist;
        }
    })?;

    match app_state().searching {
        true => rsx!(
            div { class: "w-full flex flex-row justify-center",
                img { class: "size-40", src: SEARCHING_GIF }
            }
        ),
        false => rsx!(
            div { class: "w-[100%] flex flex-col p-2 gap-2",
                p { class: "text-md font-bold text-center opacity-60", "Generated Playlist" }
                div { class: "flex flex-row flex-wrap justify-between gap-1",
                    for song in app_state().current_playlist.iter() {
                        div { class: "flex flex-col",
                            div {
                                class: "relative size-45 glass shadow-lg rounded-box",
                                background_image: format!("url({})", song.cover_art.clone()),
                                background_size: "cover",
                                // PlayPause {} // TODO: to be implemented...
                            }
                            div {
                                div { class: "truncate w-40", "{song.artist}" }
                                div { class: "text-xs truncate w-40 uppercase font-semibold opacity-60",
                                    "{song.title}"
                                }
                            }
                        }
                    }
                }
            }
        ),
    }
}

#[component]
fn PlayPause() -> Element {
    rsx!(
        label { class: "btn btn-circle bg-white h-8 w-8 absolute bottom-0 right-0 m-2 swap swap-rotate",
            input { class: "bg-transparent", r#type: "checkbox" }
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                version: "1.0",
                "viewBox": "0 0 50.000000 50.000000",
                class: "size-10 swap-off",
                g {
                    transform: "translate(0.000000,50.000000) scale(0.100000,-0.100000)",
                    fill: "#000000",
                    stroke: "none",
                    path { d: "M155 456 c-60 -28 -87 -56 -114 -116 -36 -79 -19 -183 42 -249 33 -36 115 -71 167 -71 52 0 134 35 167 71 34 37 63 110 63 159 0 52 -35 134 -71 167 -37 34 -110 63 -159 63 -27 0 -65 -10 -95 -24z m138 -167 c31 -18 57 -36 57 -39 0 -3 -26 -21 -57 -39 -32 -19 -68 -41 -80 -49 l-23 -14 0 102 0 102 23 -14 c12 -8 48 -30 80 -49z" }
                }
            }

            svg {
                version: "1.0",
                "viewBox": "0 0 50.000000 50.000000",
                xmlns: "http://www.w3.org/2000/svg",
                class: "size-10 swap-on",
                g {
                    stroke: "none",
                    fill: "#000000",
                    transform: "translate(0.000000,50.000000) scale(0.100000,-0.100000)",
                    path { d: "M155 456 c-60 -28 -87 -56 -114 -116 -36 -79 -19 -183 42 -249 33 -36 115 -71 167 -71 52 0 134 35 167 71 34 37 63 110 63 159 0 52 -35 134 -71 167 -37 34 -110 63 -159 63 -27 0 -65 -10 -95 -24z m65 -206 c0 -89 0 -90 -25 -90 -25 0 -25 1 -25 90 0 89 0 90 25 90 25 0 25 -1 25 -90z m110 0 c0 -89 0 -90 -25 -90 -25 0 -25 1 -25 90 0 89 0 90 25 90 25 0 25 -1 25 -90z" }
                }
            }
        }
    )
}
