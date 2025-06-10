use dioxus::{logger::tracing::info, prelude::*};
use strum::IntoEnumIterator;

use crate::{
    libs::db::{get_app_settings, save_app_settings},
    models::{MusicEra, MusicGenre},
    site_router::Route,
};

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub account_name: String,
    pub music_era: String,
    pub music_genre: String,
    pub provider: String,
}

#[component]
pub fn Settings() -> Element {
    let app_settings = use_server_future(move || {
        async move {
            let state = get_state_from_server(); 

            let mut db = state.db.get().unwrap();
            get_app_settings(&mut db)
        }
    })?;
    
    let app_settings = app_settings.unwrap();

    let mut saving = use_signal(|| false);
    let mut account_name: Signal<String> = use_signal(|| app_settings.user_name.clone());
    let mut music_era: Signal<MusicEra> = use_signal(|| {
        app_settings
            .music_era
            .parse::<MusicEra>()
            .unwrap_or(MusicEra::Any)
    });
    let mut music_genre: Signal<MusicGenre> = use_signal(|| {
        app_settings
            .music_genre
            .parse::<MusicGenre>()
            .unwrap_or(MusicGenre::Any)
    });
    let mut provider: Signal<String> = use_signal(|| app_settings.provider.clone());

    rsx!(
        div { class: "flex flex-col items-center mt-10 pt-[env(safe-area-inset-bottom)]",
            "Settings"
            Back {}
            div { class: "flex flex-col gap-4 mt-8 p-8 w-full",
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Account Name" }
                    input {
                        class: "input",
                        required: "false",
                        r#type: "text",
                        placeholder: "general-user",
                        inputmode: "text",
                        value: account_name(),
                        onchange: move |ev| account_name.set(ev.value()),
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Music Era" }
                    select {
                        class: "select",
                        onchange: move |ev| music_era.set(ev.value().parse::<MusicEra>().unwrap()),
                        for variant in MusicEra::iter() {
                            option {
                                selected: variant == music_era(),
                                value: variant.to_string(),
                                "{variant}"
                            }
                        }
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Music Genre" }
                    select {
                        class: "select",
                        onchange: move |ev| music_genre.set(ev.value().parse::<MusicGenre>().unwrap()),
                        for variant in MusicGenre::iter() {
                            option {
                                selected: variant == music_genre(),
                                value: variant.to_string(),
                                "{variant}"
                            }
                        }
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Provider" }
                    select {
                        class: "select",
                        onchange: move |ev| provider.set(ev.value()),
                        option {
                            selected: provider() == "Spotify",
                            value: "Spotify",
                            "Spotify"
                        }
                        option {
                            selected: provider() == "Apple Music",
                            value: "Apple Music",
                            "Apple Music"
                        }
                    }
                }
                div { class: "flex flex-row justify-center mt-10",
                    button {
                        aria_label: "save-button",
                        onclick: move |_| async move {
                            saving.set(true);
                            let app = AppSettings {
                                account_name: account_name(),
                                music_era: music_era().to_string(),
                                music_genre: music_genre().to_string(),
                                provider: provider(),
                            };
                            save_app_settings(&app).expect("failed to save app settings");
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            saving.set(false);
                        },
                        class: "btn btn-lg w-[100%] text-white bg-black border-0 shadow-lg",
                        if saving() {
                            span { class: "loading loading-spinner" }
                        }
                        span { "Save" }
                    }
                }
            }
            div { class: "absolute bottom-0 flex flex-row justify-center w-full",
                "made with ❤️ by"
                b { class: "ml-1 underline",
                    a { href: "https://github.com/saent-x", "devtor" }
                }
            }
        }
    )
}

#[component]
fn Back() -> Element {
    let navigator = navigator();

    rsx!(
        button {
            class: "btn btn-circle absolute bg-white left-0 m-4",
            onclick: move |_| {
                navigator.push(Route::Home {});
            },
            svg {
                "stroke-width": "1.5",
                "viewBox": "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                stroke: "currentColor",
                class: "size-6",
                path {
                    d: "M9 15 3 9m0 0 6-6M3 9h12a6 6 0 0 1 0 12h-3",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                }
            }
        }
    )
}
