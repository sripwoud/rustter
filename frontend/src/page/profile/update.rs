#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use web_sys::HtmlInputElement;
use crate::elements::KeyedNotifications;
use crate::util::document;

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
        None => rsx! { div { "No image uploaded"}}
    };

    render! {
        div {
            class: "flex flex-row justify-center",
            image_data
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
