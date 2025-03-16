use dioxus::prelude::*;

use crate::{libs::llm_serverfns::search_playlist, prelude::AppState};

#[component]
pub fn LLM_Chatbot() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let mut expression = use_signal(String::new);
    let mut running = use_signal(|| false);
    
    let on_chat_click = move |ev: Event<MouseData>| async move {
        ev.prevent_default();
        
        running.set(true);
        app_state.write().searching = true;
        
        let results = search_playlist(expression()).await
            .expect("failed to retrieve playlist"); //TODO: handle error
        
        app_state.write().current_playlist = results;
        app_state.write().searching = false;
        running.set(false);
    };
    
    rsx!{
        label { class: "input w-[100%] h-12 glass shadow-md bg-white rounded-full",
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
                placeholder: "state of mind...",
                enterkeyhint: "go",
                value: expression(),
                inputmode: "text",
                onchange: move |ev| expression.set(ev.value()),
            }
            button {
                onclick: on_chat_click,
                aria_label: "send-button",
                disabled: running(),
                class: "btn btn-circle bg-black border-0 shadow-lg glass",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "white",
                    "viewBox": "0 0 24 24",
                    class: "size-4",
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "M6 12 3.269 3.125A59.769 59.769 0 0 1 21.485 12 59.768 59.768 0 0 1 3.27 20.875L5.999 12Zm0 0h7.5",
                    }
                }
            }
        }
    }
}