use dioxus::prelude::*;

use crate::prelude::T3;

#[component]
pub fn MediaControl() -> Element {
    rsx!(
        div { class: "flex flex-row items-center justify-between w-[100%] bg-white p-0.5 rounded-full shadow-md glass mb-2",
            div { class: "flex flex-row gap-2 items-center",
                div {
                    div {
                        "aria-valuenow": "50",
                        role: "progressbar",
                        style: "--value:50; --size:2.2rem; --thickness: 3px;",
                        class: "radial-progress",
                        img {
                            alt: "current-song-img",
                            src: T3,
                            class: "size-7 rounded-full",
                        }
                    }
                }
                div {
                    div { class: "text-[15px]", "Dio Lupa" }
                    div { class: "text-[12px] uppercase font-semibold opacity-60", "Remaining Reason" }
                }
            }
            div {
                button {
                    "aria-label": "previous",
                    class: "btn btn-square btn-ghost",
                    svg {
                        "viewBox": "0 0 24 24",
                        stroke: "currentColor",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "black",
                        "stroke-width": "1.5",
                        class: "size-6",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M21 16.811c0 .864-.933 1.406-1.683.977l-7.108-4.061a1.125 1.125 0 0 1 0-1.954l7.108-4.061A1.125 1.125 0 0 1 21 8.689v8.122ZM11.25 16.811c0 .864-.933 1.406-1.683.977l-7.108-4.061a1.125 1.125 0 0 1 0-1.954l7.108-4.061a1.125 1.125 0 0 1 1.683.977v8.122Z",
                        }
                    }
                }
                button {
                    "aria-label": "play-pause",
                    class: "btn btn-square btn-ghost",
                    svg {
                        fill: "black",
                        xmlns: "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "size-6",
                        path {
                            "stroke-linejoin": "round",
                            "stroke-linecap": "round",
                            d: "M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z",
                        }
                    }
                }
                button { "aria-label": "next", class: "btn btn-square btn-ghost",
                    svg {
                        fill: "black",
                        stroke: "currentColor",
                        "viewBox": "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        "stroke-width": "1.5",
                        class: "size-6",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M3 8.689c0-.864.933-1.406 1.683-.977l7.108 4.061a1.125 1.125 0 0 1 0 1.954l-7.108 4.061A1.125 1.125 0 0 1 3 16.811V8.69ZM12.75 8.689c0-.864.933-1.406 1.683-.977l7.108 4.061a1.125 1.125 0 0 1 0 1.954l-7.108 4.061a1.125 1.125 0 0 1-1.683-.977V8.69Z",
                        }
                    }
                }
            }
        }
    )
}
