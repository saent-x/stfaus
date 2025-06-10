use dioxus::prelude::*;

use crate::site_router::Route;

#[derive(PartialEq, Clone, Props)]
pub struct TabProps {
    pub tab_content_1: Element,
    pub tab_content_2: Element,
}

#[component]
pub fn Tabs(content: TabProps) -> Element {
    rsx!(
        div { class: "tabs rounded-none mt-5 tabs-box justify-center w-[100%] bg-white",
            input {
                "aria-label": "new",
                name: "my_tabs_1",
                checked: true,
                r#type: "radio",
                class: "tab h-7 checked:!bg-black checked:!text-secondary-content",
            }
            div { class: "tab-content w-full mt-1 bg-white", {content.tab_content_1} }
            input {
                name: "my_tabs_1",
                r#type: "radio",
                "aria-label": "saved",
                class: "tab h-7 checked:!bg-black checked:!text-secondary-content",
            }
            div { class: "tab-content w-full mt-5 bg-white", {content.tab_content_2} }
        }
        Export {}
        Settings {}
    )
}

#[component]
fn Settings() -> Element {
    let navigator = navigator();

    rsx!(
        button {
            class: "btn btn-circle absolute bg-white right-0 m-4",
            onclick: move |_| {
                navigator.push(Route::Settings {});
            },
            svg {
                "stroke-width": "1.5",
                "viewBox": "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                stroke: "currentColor",
                class: "size-6",
                path {
                    d: "M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                }
                path {
                    d: "M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                }
            }
        }
    )
}

#[component]
fn Export() -> Element {
    rsx!(
        button {
            class: "btn btn-circle absolute bg-white left-0 m-4",
            "onclick": "my_modal_1.showModal()",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                "viewBox": "0 0 24 24",
                "stroke-width": "1.5",
                stroke: "currentColor",
                fill: "none",
                class: "size-6",
                path {
                    d: "M9 8.25H7.5a2.25 2.25 0 0 0-2.25 2.25v9a2.25 2.25 0 0 0 2.25 2.25h9a2.25 2.25 0 0 0 2.25-2.25v-9a2.25 2.25 0 0 0-2.25-2.25H15m0-3-3-3m0 0-3 3m3-3V15",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                }
            }
        }
        dialog { class: "modal", id: "my_modal_1",
            div { class: "modal-box",
                h3 { class: "text-lg font-bold", "Export Playlist" }
                fieldset { class: "fieldset mt-5",
                    legend { class: "fieldset-legend", "Playlist Title" }
                    input {
                        class: "input",
                        required: "false",
                        r#type: "text",
                        placeholder: "general-user",
                        inputmode: "text",
                                        // value: account_name(),
                    // onchange: move |ev| account_name.set(ev.value()),
                    }
                }
                div { class: "modal-action",
                    form { method: "dialog",
                        button { class: "btn", "Close" }
                    }
                    button { class: "btn bg-black text-white", "Export" }
                }
            }
        }
    )
}
