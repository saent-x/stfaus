use dioxus::{logger::tracing::info, prelude::*};

use crate::{prelude::{AppState, CURRENT_PLAYLIST, CURRENT_PROMPT}, services::search_playlist};

#[component]
pub fn LLM_Chatbot() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let mut expression = use_signal(String::new);

    let on_chat_click = move |ev: Event<KeyboardData>| async move {
        if ev.key() == Key::Enter{
            ev.prevent_default();
            
            app_state.write().searching = true;
            let results = search_playlist(&crate::models::UserPromptRequest { prompt: expression() })
                .await
                .expect("failed to retrieve playlist"); //TODO: handle error
                    
            *CURRENT_PLAYLIST.write() = results;
            *CURRENT_PROMPT.write() = expression();
            
            app_state.write().searching = false;
            expression.set("".to_string());            
        }
    };

    rsx! {
        label { 
            class: "input lg:w-[800px] min-w-[300px] md:w-[800px] h-12 focus-within:!outline focus-within:!outline-[0px] glass !border-2 !border-black-800 shadow-md bg-white rounded-full",
            svg {
                "viewBox": "0 0 24 24",
                "stroke-width": "1.5",
                fill: "none",
                stroke: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                class: "size-6",
                path {
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    d: "M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 0 1 .865-.501 48.172 48.172 0 0 0 3.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0 0 12 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018Z",
                }
            }
            input {
                required: "false",
                r#type: "search",
                placeholder: "Enter your playlist description...",
                value: expression(),
                onkeypress: on_chat_click,
                disabled: app_state().searching,
                inputmode: "text",
                oninput: move |ev| expression.set(ev.value()),
            }
        }
    }
}
