use dioxus::prelude::*;

use crate::components::{
    generated_playlist::GeneratedPlaylist,
    llm_chatbox::LLM_Chatbot,
    // media_control::MediaControl,
    tabs::{TabProps, Tabs},
};

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "h-screen flex flex-col overflow-y-scroll lg:px-[200px] md:px-[100px] px-[10px]",
            div { class: "flex flex-row flex-grow justify-center mt-4 pt-[env(safe-area-inset-bottom)] w-[100%] mx-auto pb-40",
                Tabs {
                    content: TabProps {
                        tab_content_1: rsx! {
                            GeneratedPlaylist {}
                        },
                        tab_content_2: rsx! {
                            div {
                                p { class: "text-center text-sm mt-[40px] text-[#4A709C]",
                                    i { "No saved playlist yet..." }
                                }
                            }
                        },
                    },
                }
            }
            div { class: "flex flex-col md:pb-[0px] bg-white/90 lg:pb-[50px] p-4 items-center mt-auto w-[100%] bottom-0 left-0 right-0 fixed",
                LLM_Chatbot {}
                i { class: "mt-5 text-[#4A709C] text-sm",
                    "made with ❤️ by devtor"
                }
            }
        }
    }
}
