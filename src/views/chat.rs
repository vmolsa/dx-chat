use std::time::Instant;

use crate::{Author, Message, AUTHORS, ROOMS};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::{LdMoveDown, LdSendHorizontal}, Icon};
use uuid::Uuid;

#[component]
pub fn Chat(id: Uuid) -> Element {
    let local_user = use_context::<Author>();

    let mut input_line: Signal<usize> = use_signal(|| 0);
    let mut input_lines: Signal<Vec<String>> = use_signal(|| vec![String::new()]);
    let mut input_cursor: Signal<usize> = use_signal(|| 0);
    let mut last_message: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let mut message_viewer: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let mut chat_input: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let mut show_scroll = use_signal(|| false);

    let rooms = ROOMS.read();
    let authors = AUTHORS.read();

    let Some(room) = rooms.get(&id) else {
        return rsx! {
            div {
                "EMPTY ROOM"
            }
        }
    };

    let messages = room.messages.iter().map(|(_, msg)| {
        let (side_class, name, avatar_url) = if msg.author == local_user.id {
            (
                "chat chat-end",
                local_user.username.as_str(),
                local_user.avatar.as_ref()
                        .and_then(|url| Some(url.as_str()))
            )
        } else {
            authors
                .get(&msg.author)
                .map(|author| (
                    "chat chat-start",
                    author.username.as_str(),
                    author.avatar.as_ref()
                        .and_then(|url| Some(url.as_str()))
                ))
                .unwrap_or((
                    "chat chat-start",
                    "Unknown", 
                    None
                ))
        };

        rsx! {
            div { 
                class: "{side_class} gap-2",
                div { 
                    class: "chat-image avatar",
                    
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
                    class: "chat-bubble", 
                    for line in msg.content.split('\n') {
                        p {
                            span {
                                "{line}"
                            }
                        }
                    }
                }
                div { 
                    class: "chat-header",
                    "{name}",
                }
                div {
                    class: "chat-footer opacity-50",
                    time { 
                        class: "text-xs text-secondary opacity-50", 
                        "{msg.id.elapsed().as_secs()}s ago" 
                    }
                }
            }
        }
    });

    let update_viewer = move || async move {
        if let Some(viewer) = message_viewer.cloned() {
            if let Ok(scroll) = viewer.get_scroll_size().await {
                if let Ok(offset) = viewer.get_scroll_offset().await {
                    if let Ok(rect) = viewer.get_client_rect().await {
                        let current = scroll.height - offset.y;

                        if current <= rect.size.height {
                            show_scroll.set(false);
                        } else {
                            show_scroll.set(true);
                        }
                    }
                }
            }
        }
    };

    let scroll_to_last = move |smooth: bool| async move {
        if let Some(message) = last_message.cloned() {
            let _ = message.scroll_to(if smooth {
                ScrollBehavior::Smooth
            } else {
                ScrollBehavior::Instant
            }).await;
        }
    };

    let focus_chat_input = move || async move {
        if let Some(chat_input) = chat_input.cloned() {
            let _ = chat_input.set_focus(true).await;
        }
    };

    let send_message = move |content: String| async move {
        let ts = Instant::now();

        {
            let mut rooms = ROOMS.write();

            if let Some(room) = rooms.get_mut(&id) {
                room.messages.insert(ts, Message {
                    id: ts,
                    author: local_user.id,
                    content,
                });
            }
        }
    };

    let input_handler = move |evt: Event<KeyboardData>| async move {
        focus_chat_input().await;

        let mut idx = input_line.write();
        let mut lines = input_lines.write();
        let mut cursor = input_cursor.write();

        match evt.key() {
            Key::Enter if evt.modifiers().shift() => {
                evt.prevent_default();
                // split at cursor
                let rest = lines[*idx].split_off(*cursor);
                
                lines.insert(*idx + 1, rest);

                *idx += 1;
                *cursor = 0;
            }
            Key::Enter => {
                send_message(lines.join("\n")).await;

                lines.clear();
                lines.push(String::new());

                *idx = 0;
                *cursor = 0;
            }
            Key::Backspace => {
                if *cursor > 0 {
                    let pos = *cursor - 1;

                    lines[*idx].remove(pos);
                    *cursor = pos;
                } else if *idx > 0 {
                    // merge up
                    let tail = lines.remove(*idx);
                    
                    *idx -= 1;
                    lines[*idx - 1].push_str(&tail);
                    *cursor = 0;
                }
            }
            Key::Delete => {
                if *cursor < lines[*idx].len() {
                    lines[*idx].remove(*cursor);
                } else if *idx + 1 < lines.len() {
                    let next = lines.remove(*idx + 1);
                    lines[*idx].push_str(&next);
                    *cursor = 0;
                }
            }
            Key::Character(ch) => {
                if ch == "v" && (evt.modifiers().ctrl() || evt.modifiers().meta()) {
                    evt.prevent_default();
                } else {
                    lines[*idx].insert_str(*cursor, ch.as_str());
                    *cursor += ch.len();
                }
            }
            Key::ArrowLeft => {
                if *cursor > 0 {
                    *cursor -= 1;
                } else if *idx > 0 {
                    *idx -= 1;
                    *cursor = 0;
                }
            }
            Key::ArrowRight => {
                if *cursor < lines[*idx].len() {
                    *cursor += 1;
                } else if *idx + 1 < lines.len() {
                    *idx += 1;
                    *cursor = 0;
                }
            }
            Key::ArrowUp if *idx > 0 => {
                *idx -= 1;
                *cursor = 0;
            }
            Key::ArrowDown if *idx + 1 < lines.len() => {
                *idx += 1;
                *cursor = 0;
            }
            Key::Home => {
                *cursor = 0;
            }
            Key::End => {
                *cursor = lines[*idx].len();
            }
            _ => {}
        }
    };

    let input_paste = move |evt: Event<ClipboardData>| async move {
        evt.stop_propagation();

        let data = evt.data();
        
        if let Some(content) = data.downcast::<String>() {
            let mut lines = input_lines.write();
            let mut cursor = input_cursor.write();
            let mut line_idx = input_line();

            // keep the tail of the current line
            let rest = lines[line_idx].split_off(*cursor);

            // iterate over each chunk, splitting on '\n'
            let mut iter = content.split('\n').peekable();

            // first chunk: always append into current line
            if let Some(first) = iter.next() {
                lines[line_idx].push_str(first);
                *cursor += first.len();
            }

            // for all following chunks...
            while let Some(chunk) = iter.next() {
                if iter.peek().is_some() {
                    // intermediate line: insert a new empty line
                    lines.insert(line_idx + 1, chunk.to_string());
                    line_idx += 1;
                    input_line.set(line_idx);
                    *cursor = chunk.len();
                } else {
                    // last chunk: create the new line and re-append the old remainder
                    lines.insert(line_idx + 1, format!("{chunk}{rest}"));
                    input_line.set(line_idx + 1);
                    *cursor = chunk.len();
                }
            }

            // if there was no '\n' at all, re-append the remainder
            if content.find('\n').is_none() {
                lines[line_idx].push_str(&rest);
            }
        }
    };

    rsx! {
        div {
            onresize: move |_| async move {
                update_viewer().await;
            },
            onvisible: move |_| async move {
                update_viewer().await;
            },
            onmounted: move |_| async move {
                update_viewer().await;
                focus_chat_input().await;
            },
            class: "flex flex-col w-full gap-4 p-2 h-full", 
            div {
                onmounted: move |evt| async move {
                    message_viewer.set(Some(evt.data()));
                    update_viewer().await;
                },
                onscroll: move |_| async move {
                    update_viewer().await;
                },
                class: "flex flex-col card gap-8 p-4 pr-8 grow text-wrap overflow-y-auto justify-end",
                if show_scroll() && last_message().is_some() {
                    div {
                        class: "flex flex-col btn btn-ghost hover:btn-outline fixed bottom-20 left-[50%] w-24 right-[50%]",
                        button {  
                            onclick: move |_| async move {
                                scroll_to_last(true).await;
                                focus_chat_input().await;
                            },
                            Icon {
                                class: "stroke-slate-500 dark:stroke-slate-300 opacity-25 group-hover:opacity-100",
                                width: 20,
                                height: 20,
                                icon: LdMoveDown,
                            }
                        }
                    }
                }
                for message in messages {
                    {message}
                }
                div {
                    class: "h-4",
                    onmounted: move |evt| last_message.set(Some(evt.data())),
                }
            }
            div {
                class: "flex flex-col items-start mt-4",
                label {
                    tabindex: 0,
                    autofocus: true,
                    onkeydown: input_handler,
                    onpaste: input_paste,
                    onmounted: move |evt| chat_input.set(Some(evt.data())),
                    class: "flex flex-row gap-4 w-full group border-1 items-center border-accent outline-accent ring-accent rounded-2xl outline-0 p-4",
                    Icon {
                        class: "stroke-slate-700 dark:stroke-slate-400 opacity-25 group-hover:opacity-100",
                        width: 20,
                        height: 20,
                        icon: LdSendHorizontal,
                    }
                    div {
                        class: "flex flex-col",
                        for (index, line) in input_lines().iter().enumerate() {
                            ChatInputLine {
                                index: index,
                                line: line,
                                input_line: input_line,
                                input_cursor: input_cursor,
                            }   
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ChatInputLine(index: usize, line: String, mut input_line: Signal<usize>, input_cursor: Signal<usize>) -> Element {
    if index == input_line() {
        let pos = input_cursor();
        let (before, after) = line.split_at(pos);
        
        rsx! {
            p {
                span { 
                    "{before}" 
                }
                span { 
                    class: "animate-ping font-medium text-xl", 
                    "|" 
                }
                span { 
                    "{after}"
                }
            }
        }
    } else {
        rsx! {
            p {
                onclick: move |evt| {
                    dbg!(evt);

                    input_line.set(index);
                    input_cursor.set(0);
                },
                span {
                    "{line}"
                }
            }
        }
    }
}