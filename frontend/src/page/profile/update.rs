#![allow(non_snake_case)]

use crate::elements::{KeyedNotifications, KeyedNotificationsBox};
use crate::prelude::*;
use crate::util::document;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use rustter_domain::{ConstrainedText, ConstrainedUserFacingError, UserFacingError};
use web_sys::HtmlInputElement;

#[derive(Clone, Debug)]
enum PreviewImageData {
    DataUrl(String),
    Remote(String),
}

#[derive(Clone, Debug, Default)]
pub struct PageState {
    form_errors: KeyedNotifications,
    display_name: String,
    email: String,
    password: String,
    password_confirmation: String,
    profile_image: Option<PreviewImageData>,
}

// TODO: extract in components (used by post create image post and update profile)
#[inline_props]
pub fn ImageInput(cx: Scope, state: UseRef<PageState>) -> Element {
    let toaster = use_toaster(cx);

    render! {
        div {
            label {
                r#for:"image-input",
                "Upload Image"
            },
            input {
                class: "w-full",
                id: "image-input",
                r#type: "file",
                accept: "image/*",
                oninput: |_| {
                    to_owned![state,toaster];
                    async move {
                        use gloo_file::{File, futures::read_as_data_url};
                        use wasm_bindgen::JsCast;

                        let el = document().get_element_by_id("image-input").unwrap().unchecked_into::<HtmlInputElement>();
                        let file: File = el.files().unwrap().get(0).unwrap().into();
                        match read_as_data_url(&file).await {
                            Ok(data_url) => {
                                state.with_mut(|state|{
                                    state.profile_image = Some(PreviewImageData::DataUrl(data_url));
                                });
                            }
                            Err(e) => toaster.write().error(format!("Failed to read image: {e}"), None)
                        }
                    }
                }
            }
        }
    }
}

#[inline_props]
pub fn ImagePreview(cx: Scope, state: UseRef<PageState>) -> Element {
    let image_data = state.with(|state| state.profile_image.clone());
    let img_el = |img_src| {
        rsx! {
            img {
                class: "profile-portrait-lg",
                src: "{img_src}"
            }
        }
    };

    let image_data = match image_data {
        Some(PreviewImageData::DataUrl(ref data)) => img_el(data),
        Some(PreviewImageData::Remote(ref url)) => img_el(url),
        None => rsx! { div { "No image uploaded"}},
    };

    render! {
        div {
            class: "flex flex-row justify-center",
            image_data
        }
    }
}

#[inline_props]
pub fn EmailInput(cx: Scope, state: UseRef<PageState>) -> Element {
    use rustter_domain::user::Email;

    render! {
        div {
            label {
                r#for:"email",
                div {
                class:"flex flex-row justify-between",
                    span{"Email Address"},
                }
            },
            input {
                class:"input-field",
                id:"email",
                placeholder: "user@domain.com",
                value:"{state.read().email}",
                oninput:move|ev|{
                    match Email::new(&ev.value) {
                       Ok(_) => {
                            state.with_mut(|state|state.form_errors.remove("bad-email"));
                        },
                        Err(e) => {
                            state.with_mut(|state|state.form_errors.set("bad-email", e.formatted_error()));
                        }
                    }
                    state.with_mut(|state| state.email = ev.value.clone());
                }
            }
        }
    }
}

#[inline_props]
pub fn DisplayNameInput(cx: Scope, state: UseRef<PageState>) -> Element {
    use rustter_domain::user::DisplayName;

    let wrong_len = maybe_class!(
        "err-text-color",
        state.read().display_name.len() > DisplayName::MAX_CHARS
    );
    render! {
        div {
            label {
                r#for:"display-name",
                div {
                class:"flex flex-row justify-between",
                    span{"Display Name"},
                    span {
                        class: "text-right {wrong_len}",
                        "{state.read().display_name.len()}/{DisplayName::MAX_CHARS}"
                    }
                }
            },
            input {
                class:"input-field",
                id:"display-name",
                placeholder: "John Doe",
                value:"{state.read().display_name}",
                oninput:move|ev|{
                    match DisplayName::new(&ev.value) {
                       Ok(_) => {
                            state.with_mut(|state|state.form_errors.remove("bad-display-name"));
                        },
                        Err(e) => {
                            state.with_mut(|state|state.form_errors.set("bad-display-name", e.formatted_error()));
                        }
                    }
                    state.with_mut(|state| state.display_name = ev.value.clone());
                }
            }
        }
    }
}


pub fn UpdateProfile(cx: Scope) -> Element {
    let page_state = use_ref(cx, PageState::default);
    let nav = use_navigator(cx);

    cx.render(rsx! {
        div {
            class: "flex flex-col w-full gap-3",
            ImagePreview {state: page_state.clone()},
            ImageInput { state:page_state.clone()},
            DisplayNameInput {state: page_state.clone()},
            EmailInput {state: page_state.clone()},
            KeyedNotificationsBox { notifications: page_state.clone().read().form_errors.clone() },
            div {
                class: "flex flex-row justify-end gap-3",
                button {
                    class:"btn",
                    prevent_default: "onclick",
                    onclick: move |_| {nav.go_back();},
                    "Cancel"
                },
                button {
                    class:"btn",
                    onclick: move |_| {nav.go_back();},
                    "Submit"
                }
            }
        }
    })
}
