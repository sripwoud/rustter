use crate::elements::use_toaster;
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub message: String,
    pub headline: String,
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        use rustter_domain::post::{Headline, Message};

        if Message::new(&self.message).is_err() {
            return false;
        }
        if !self.headline.is_empty() && Headline::new(&self.headline).is_err() {
            return false;
        }
        true
    }
}

#[inline_props]
pub fn MessageInput(cx: Scope, state: UseRef<PageState>) -> Element {
    use rustter_domain::post::Message;
    let wrong_len = maybe_class!(
        "err-text-color",
        state.read().message.len() > Message::MAX_CHARS || state.read().message.is_empty()
    );

    render! {
        div {
            label {
                r#for:"example",
                div {
                class:"flex flex-row justify-between",
                    span{"Message"},
                    span {
                        class: "text-right {wrong_len}",
                        "{state.read().message.len()}/{Message::MAX_CHARS}"
                    }
                }
            },
            textarea {
                class:"input-field {wrong_len}",
                id:"message",
                rows:5,
                value:"{state.read().message}",
                oninput:move|ev|{
                    state.with_mut(|state|{
                        state.message = ev.data.value.clone()
                    });
                }
            }
        }
    }
}

#[inline_props]
pub fn HeadlineInput(cx: Scope, state: UseRef<PageState>) -> Element {
    use rustter_domain::post::Headline;
    let wrong_len = maybe_class!(
        "err-text-color",
        state.read().headline.len() > Headline::MAX_CHARS
    );

    render! {
        div {
            label {
                r#for:"example",
                div {
                class:"flex flex-row justify-between",
                    span{"Headline"},
                    span {
                        class: "text-right {wrong_len}",
                        "{state.read().headline.len()}/{Headline::MAX_CHARS}"
                    }
                }
            },
            input {
                class:"input-field {wrong_len}",
                id:"headline",
                value:"{state.read().headline}",
                oninput:move|ev|{
                    state.with_mut(|state|{
                        state.headline = ev.data.value.clone()
                    });
                }
            }
        }
    }
}

pub fn NewChatPost(cx: Scope) -> Element {
    let state = use_ref(cx, PageState::default);
    let nav = use_navigator(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let submit_btn_style = maybe_class!("button-disabled", !state.read().can_submit());

    let onclick = async_handler!(
        &cx,
        [api_client, state, nav, toaster],
        move |_| async move {
            use rustter_endpoint::post::endpoint::{NewPost, NewPostOk};
            use rustter_endpoint::post::types::{Chat, NewPostOptions};

            let request = {
                use rustter_domain::post::{Headline, Message};
                NewPost {
                    content: Chat {
                        headline: {
                            let headline = &state.read().headline;
                            if headline.is_empty() {
                                None
                            } else {
                                Some(Headline::new(headline).unwrap())
                            }
                        },
                        message: Message::new(&state.read().message).unwrap(),
                    }
                    .into(),
                    options: NewPostOptions::default(),
                }
            };

            let response = post_json!(<NewPostOk>, api_client, request);
            match response {
                Ok(_) => {
                    toaster.write().success("Posted!", None);
                    nav.push(Route::Home {});
                }
                Err(_e) => {
                    toaster.write().error("Failed to post: {e}!", None);
                }
            }
        }
    );

    render! {
        div {
            class: "flex flex-col gap-4",
            prevent_default: "onsubmit",
            HeadlineInput {
                state: state.clone(),
            }
            MessageInput {
                state: state.clone(),
            },
            button {
                class: "btn {submit_btn_style}",
                r#type: "submit",
                disabled: !state.read().can_submit(),
                onclick: onclick,
                "Post"
            }

        }
    }
}
