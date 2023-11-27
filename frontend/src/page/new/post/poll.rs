use crate::elements::use_toaster;
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use rustter_domain::ids::PollChoiceId;
use rustter_domain::post::{PollChoiceDescription, PollHeadline};
use rustter_domain::ConstrainedText;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub headline: String,
    pub poll_choices: BTreeMap<usize, String>,
    pub next_id: usize,
}

impl Default for PageState {
    fn default() -> Self {
        Self {
            headline: String::new(),
            poll_choices: {
                let mut map = BTreeMap::new();
                map.insert(0, String::new());
                map.insert(1, String::new());
                map
            },
            next_id: 2,
        }
    }
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        if PollHeadline::new(&self.headline).is_err() {
            return false;
        }

        if self.poll_choices.len() < 2 {
            return false;
        }

        if self.poll_choices.values().any(|choice| choice.is_empty()) {
            return false;
        }

        true
    }

    pub fn push_choice<T: Into<String>>(&mut self, choice: T) {
        self.poll_choices.insert(self.next_id, choice.into());
        self.next_id += 1;
    }

    pub fn replace_choice<T: Into<String>>(&mut self, id: usize, choice: T) {
        self.poll_choices.insert(id, choice.into());
    }
}

#[inline_props]
pub fn HeadlineInput(cx: Scope, state: UseRef<PageState>) -> Element {
    use rustter_domain::post::PollHeadline;
    use rustter_domain::ConstrainedText;

    let wrong_len = maybe_class!(
        "err-text-color",
        state.read().headline.len() > PollHeadline::MAX_CHARS || state.read().headline.is_empty()
    );

    render! {
        div {
            label {
                r#for:"headline",
                div {
                class:"flex flex-row justify-between",
                    span{"Headline (optional)"},
                    span {
                        class: "text-right {wrong_len}",
                        "{state.read().headline.len()}/{PollHeadline::MAX_CHARS}"
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

#[inline_props]
pub fn ChoicesInput(cx: Scope, state: UseRef<PageState>) -> Element {
    let choices = state
        .read()
        .poll_choices
        .iter()
        .map(|(&key, choice)| {
            let choice = choice.clone();
            let max_chars = PollChoiceDescription::MAX_CHARS;
            let wrong_len = maybe_class!(
                "err-text-color",
                PollChoiceDescription::new(&choice).is_err()
            );

            rsx! {
                li {
                    key: "{key}",
                    div {
                        class: "grid grid-cols-[1fr_3rem_3rem] w-full gap-2 items-center h-8",
                        input {
                            class: "input-field {wrong_len}",
                            placeholder: "Choice description",
                            value: "{choice}",
                            oninput: move |ev| {
                                state.with_mut(|state| {
                                    state.replace_choice(key, ev.data.value.clone())
                                });
                            }
                        }
                        div {
                            class: "text-right {wrong_len}",
                            "{choice.len()}/{max_chars}"
                        }
                        button {
                            class: "btn p-0 h-full bg-red-400",
                            prevent_default: "onclick",
                            onclick: move |_| {
                                state.with_mut(|state| {
                                    state.poll_choices.remove(&key);
                                });
                            },
                            "X"
                        }
                    }
                }
            }
        })
        .collect::<Vec<LazyNodes>>();

    render! {
        div {
            class: "flex flex-col gap-2",
            ol {
                class: "list-decimal ml-4 flex flex-col gap-2",
                choices.into_iter()
            },
            div {
                class: "flex flex-row justify-end gap-2",
                button {
                    class: "btn bg-green-400 w-12",
                    prevent_default: "onclick",
                    onclick: move |_| {
                        state.with_mut(|state| {
                            state.push_choice(String::new());
                        });
                    },
                    "+"
                }
            }
        }
    }
}

pub fn NewPollPost(cx: Scope) -> Element {
    let state = use_ref(cx, PageState::default);
    let nav = use_navigator(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let submit_btn_style = maybe_class!("button-disabled", !state.read().can_submit());

    let onclick = async_handler!(
        &cx,
        [api_client, state, nav, toaster],
        move |_| async move {
            use rustter_domain::post::PollHeadline;
            use rustter_endpoint::post::{
                endpoint::{NewPost, NewPostOk},
                types::{ImageKind, NewPostOptions, Poll, PollChoice},
            };

            let request = {
                NewPost {
                    content: Poll {
                        headline: PollHeadline::new(&state.read().headline).unwrap(),
                        choices: state
                            .read()
                            .poll_choices
                            .values()
                            .map(|choice| PollChoice {
                                id: PollChoiceId::new(),
                                description: PollChoiceDescription::new(choice).unwrap(),
                                num_votes: 0,
                            })
                            .collect::<Vec<PollChoice>>(),
                        voted: None,
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
                Err(e) => {
                    toaster.write().error(format!("Failed to post: {e}!"), None);
                }
            }
        }
    );

    render! {
        AppBar {
            title: "New Poll",
        }
        div {
            class: "flex flex-col gap-4",
            prevent_default: "onsubmit",
            HeadlineInput {
                state: state.clone()
            },
            ChoicesInput {
                state: state.clone()
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
