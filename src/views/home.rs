use dioxus::prelude::*;

use crate::AUTHORS;

#[component]
pub fn Home() -> Element {
    let authors = AUTHORS.read();

    let authors_list = authors.iter()
        .map(|(_, author)| (
            "chat chat-start",
            author.username.as_str(),
            author.avatar.as_ref()
                .and_then(|url| Some(url.as_str())),
            author.bio.as_ref()
                .and_then(|bio| Some(bio.as_str()))
        ))
        .map(|(side_class, name, avatar_url, bio)| {
            rsx! {
                button {
                    class: "btn btn-outline btn-accent p-12",
                    div { 
                        class: "{side_class} gap-2",
                        div { 
                            class: "chat-image avatar !text-2xl",
                            div { 
                                class: "w-10 rounded-full",
                                if let Some(avatar_url) = avatar_url {
                                    img {
                                        alt: "{name}",
                                        src: "{avatar_url}"
                                    }
                                }
                            }
                        }
                        div { 
                            class: "chat-header",
                            "{name}",
                        }
                        if let Some(bio) = bio {
                            div {
                                class: "!text-xs opacity-50",
                                "{bio}"
                            }
                        }
                    }
                }
            }
        });

    rsx! {
        div {
            class: "flex flex-col w-fit gap-4 p-4",
            for author in authors_list {
                {author}
            }
        }
    }
}
