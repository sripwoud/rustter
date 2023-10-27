#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use rustter_domain::post::Message;

fn can_reply(message: &str) -> bool {
    message.len() < Message::MAX_CHARS && !message.is_empty()
}

#[inline_props]
pub fn MessageInput<'a>(
    cx: Scope<'a>,
    message: &'a str,
    on_input: EventHandler<'a, FormEvent>,
) -> Element {
    let wrong_len = maybe_class!("err-text-color", !can_reply(message));

    cx.render(rsx! {
        div {
            class:"flex flex-row relative",
            textarea {
                class:"input-field",
                id:"message",
                rows:3,
                value:"{message}",
                oninput:move |ev|{on_input.call(ev);},
            },
            div {
                class:"text-right {wrong_len} absolute bottom-1 right-1",
                "{message.len()}/{Message::MAX_CHARS}"
            }
        }
    })
}

#[inline_props]
pub fn QuickRespond(cx: Scope, opened: UseState<bool>) -> Element {
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);
    let message = use_state(cx, || "".to_string());

    let handle_click = async_handler!(
        &cx,
        [api_client, toaster, message, opened],
        move |_| async move {
            use rustter_endpoint::post::endpoint::{NewPost, NewPostOk};
            use rustter_endpoint::post::types::{Chat, NewPostOptions};

            let request = {
                NewPost {
                    content: Chat {
                        headline: None,
                        message: Message::new(message.get()).unwrap(),
                    }
                    .into(),
                    options: NewPostOptions::default(),
                }
            };

            let response = post_json!(<NewPostOk>, api_client, request);
            match response {
                Ok(_) => {
                    toaster.write().success("Posted!", None);
                    opened.set(false);
                }
                Err(_e) => {
                    toaster.write().error("Reply failed: {e}!", None);
                }
            }
        }
    );

    let submit_cursor = maybe_class!("cursor-not-allowed", !can_reply(message.get()));
    let submit_btn_style = maybe_class!("button-disabled", !can_reply(message.get()));

    cx.render(rsx! {
        div {
            onclick: handle_click,
            MessageInput {
                message: message.get(),
                on_input: move |ev:FormEvent| {
                    message.set(ev.data.value.clone());
                }
            },
            div {
                class:"w-full flex flex-row justify-end",
                button {
                    class: "mt-2 btn button-primary {submit_cursor} {submit_btn_style}",
                    disabled: !can_reply(message.get()),
                    "Respond"
                }
            }
        }
    })
}
