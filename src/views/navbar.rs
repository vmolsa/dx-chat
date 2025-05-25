use crate::{Route, ROOMS};
use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::FaRust;
use dioxus_free_icons::icons::ld_icons::{LdHash, LdSettings2};
use dioxus_free_icons::Icon;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let mut open_settings = use_signal(|| false);

    let rooms = ROOMS.read();

    let rooms = rooms.iter().map(|(id, room)| {
        rsx! {
            div {
                class: "flex flex-row group items-center gap-2 p-2 justify-between",
                Link {
                    to: Route::Chat { id: *id },
                    class: "flex flex-row group items-center rounded-md hover:bg-secondary w-full p-2",
                    div {
                        class: "flex flex-row items-center gap-2",
                        Icon {
                            class: "stroke-slate-800 dark:stroke-slate-500 opacity-50",
                            width: 18,
                            height: 18,
                            icon: LdHash,
                        }
                        "{room.name.as_str()}"
                    },
                },
                button {
                    class: "flex flex-row opacity-0 group-hover:opacity-100 btn btn-xs btn-ghost hover:btn-outline group",
                    onclick: move |evt| {
                        evt.prevent_default();
                        open_settings.set(true);
                    },
                    Icon {
                        class: "stroke-slate-700 dark:stroke-slate-400 opacity-25 group-hover:opacity-100",
                        width: 16,
                        height: 16,
                        icon: LdSettings2,
                    }
                }
            }
        }
    });

    rsx! {
        dialog {
            open: open_settings,
            class: "modal",
            div { 
                class: "modal-box",
                div {
                    class: "absolute top-4 right-4",
                    button {
                        onclick: move |_| open_settings.set(false),
                        class: "btn dark:text-white", 
                        "X"
                    }
                }
                h3 { class: "text-lg font-bold", "Hello!" }
                p { class: "py-4", "Press ESC key or click outside to close" }
                button {
                    onclick: move |_| open_settings.set(false),
                    class: "btn",
                    "Close"
                }
            }
            form {
                onsubmit: move |_| open_settings.set(false),
                method: "dialog",
                class: "modal-backdrop",
                button {
                    class: "w-full h-full",    
                }
            }
        }
        div {
            class: "flex flex-row w-full  m-0 h-screen max-h-screen",
            div {
                class: "flex flex-col items-start p-2 gap-4 min-w-[200px] max-w-1/4",
                div {
                    class: "flex flex-col gap-2 p-2 grow overflow-auto border-r-1 border-slate-800 dark:border-slate-500",
                    for room in rooms {
                        {room}
                    }
                }
                div {
                    class: "flex flex-col w-full",
                    div {
                        class: "flex flex-row justify-between items-center p-2",
                        button {
                            class: "flex flex-col btn btn-md btn-ghost hover:btn-outline btn-accent w-full",
                            onclick: move |_| open_settings.set(true),
                            Icon {
                                class: "fill-slate-700 dark:fill-slate-400",
                                width: 30,
                                height: 30,
                                icon: FaRust,
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-col border-red-500 p-2 grow overflow-auto",
                Outlet::<Route> {}
            }
        }        
    }
}
