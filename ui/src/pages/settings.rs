use dioxus::{logger::tracing::info, prelude::*};

use crate::{ 
    models::AppSettingsUpdate, services::{get_music_era_genre_list, get_settings, update_settings}, site_router::Route
};


#[component]
pub fn Settings() -> Element {
    let app_settings = use_resource(get_settings).suspend()?()?;
    let music_era_genre_list = use_resource(get_music_era_genre_list).suspend()?()?;
    
    let mut saving = use_signal(|| false);
    let mut user_name: Signal<String> = use_signal(|| app_settings.user_name);
    let mut music_era: Signal<String> = use_signal(|| app_settings.music_era);
    let mut music_genre: Signal<String> = use_signal(|| app_settings.music_genre);

    rsx!(
        div { class: "flex flex-col items-center pt-[30px]",
            "Settings"
            Back {}
            div { class: "flex flex-col items-center gap-4 mt-8 p-8 w-full",
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Account Name" }
                    input {
                        class: "input w-[800px]",
                        required: "false",
                        r#type: "text",
                        placeholder: "general-user",
                        inputmode: "text",
                        value: user_name(),
                        onchange: move |ev| user_name.set(ev.value()),
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Music Era" }
                    select {
                        class: "select w-[800px]",
                        onchange: move |ev| music_era.set(ev.value()),
                        for variant in music_era_genre_list.music_eras {
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
                        class: "select w-[800px]",
                        onchange: move |ev| music_genre.set(ev.value()),
                        for variant in music_era_genre_list.music_genres {
                            option {
                                selected: variant == music_genre(),
                                value: variant.to_string(),
                                "{variant}"
                            }
                        }
                    }
                }
                
                div { class: "flex flex-row justify-center mt-10",
                    button {
                        aria_label: "save-button",
                        onclick: move |_| async move {
                            saving.set(true);
                            let app = AppSettingsUpdate {
                                user_name: user_name(),
                                music_era: music_era().to_string(),
                                music_genre: music_genre().to_string(),
                            };
                            update_settings(app)
                                .await.expect("failed to save app settings");
                            
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await; // probably won't work
                            saving.set(false);
                        },
                        class: "btn btn-lg w-[300px] text-white bg-black border-0 shadow-lg",
                        if saving() {
                            span { class: "loading loading-spinner" }
                        }
                        span { "Save" }
                    }
                }
            }
            div { class: "flex flex-row justify-center w-full",
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
