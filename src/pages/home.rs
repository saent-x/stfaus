use dioxus::prelude::*;

use crate::components::{
    generated_playlist::GeneratedPlaylist,
    llm_chatbox::LLM_Chatbot,
    media_control::MediaControl,
    tabs::{TabProps, Tabs},
};

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "h-screen flex flex-col overflow-y-scroll",
            div { class: "flex flex-row flex-grow justify-center mt-4 pt-[env(safe-area-inset-bottom)] w-[100%] mx-auto pb-40",
                Tabs {
                    content: TabProps {
                        tab_content_1: rsx! {
                            GeneratedPlaylist {}
                        },
                        tab_content_2: rsx! {
                            div { "tab 2" }
                        },
                    },
                }
            }
            div { class: "flex flex-col mb-4 p-4 items-center mt-auto w-[100%] bottom-0 left-0 right-0 fixed",
                MediaControl {}
                LLM_Chatbot {}
            }
        }
    }
}
