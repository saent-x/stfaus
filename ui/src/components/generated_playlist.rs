use crate::{prelude::*, services::get_playlist};
use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn GeneratedPlaylist() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    use_resource(|| async move {
        let playlist = get_playlist().await;
        match playlist {
            Ok(data) => {
                *CURRENT_PLAYLIST.write() = data;
            },
            Err(_) => {
                *CURRENT_PLAYLIST.write() = vec![];
            }
        }
    }).suspend()?;
    
    
    match app_state().searching {
        true => rsx!(
            div { class: "w-full mt-[150px] flex flex-col items-center",
                img { class: "size-[300px]", src: SEARCHING_GIF }
                
                p { class: "text-center text-sm mt-[40px] text-[#4A709C]",
                    i { "Relax, the AI gods are curating your playlist..." }
                }
            }
        ),
        false => rsx!(
            div { class: "w-[100%] flex flex-col gap-2",
                
                div { class: "flex flex-col items-center lg:items-start md:items-start",
                    p { class: "text-2xl font-extrabold pb-[10px]", "St.Faus Generated Playlist" }
                    p { class: "text-sm", 
                        "Generated from your description: "
                        i { class: "text-bold",
                            "'{CURRENT_PROMPT()}'"
                        }
                    }
                }
                
                div { class: "flex flex-row flex-wrap justify-between gap-1 mt-[50px]",
                    for song in CURRENT_PLAYLIST() {
                        div { class: "flex flex-col mb-5 transition-transform transform hover:scale-105 cursor-pointer",
                            div {
                                class: "relative size-45 glass shadow-lg rounded-box",
                                background_image: format!("url({})", song.cover_art.clone()),
                                background_size: "cover",
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
