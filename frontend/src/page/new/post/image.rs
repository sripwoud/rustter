use crate::elements::use_toaster;
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use serde::{Deserialize, Serialize};
use crate::util::document;
use web_sys::HtmlInputElement;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub caption: String,
    pub image: Option<String>,
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        use rustter_domain::post::Caption;

        if !self.caption.is_empty() && Caption::new(&self.caption).is_err() {
            return false;
        }

        if self.image.is_none() {
            return false;
        }

        true
    }
}

#[inline_props]
pub fn ImageInput(cx:Scope, state:UseRef<PageState>) -> Element {
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
                                    state.image = Some(data_url);
                                });
                            }
                            Err(e) => toaster.write().error(format!("Failed to reading image: {e}"), None)
                        }
                    }
                }
            }
        }
    }
}

#[inline_props]
pub fn CaptionInput(cx: Scope, state: UseRef<PageState>) -> Element {
    use rustter_domain::post::Caption;
    let wrong_len = maybe_class!(
        "err-text-color",
        state.read().caption.len() > Caption::MAX_CHARS
    );

    render! {
        div {
            label {
                r#for:"caption",
                div {
                class:"flex flex-row justify-between",
                    span{"Caption (optional)"},
                    span {
                        class: "text-right {wrong_len}",
                        "{state.read().caption.len()}/{Caption::MAX_CHARS}"
                    }
                }
            },
            input {
                class:"input-field {wrong_len}",
                id:"caption",
                value:"{state.read().caption}",
                oninput:move|ev|{
                    state.with_mut(|state|{
                        state.caption = ev.data.value.clone()
                    });
                }
            }
        }
    }
}

pub fn NewImagePost(cx: Scope) -> Element {
    let state = use_ref(cx, PageState::default);
    let nav = use_navigator(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let submit_btn_style = maybe_class!("button-disabled", !state.read().can_submit());

    let onclick = async_handler!(
        &cx,
        [api_client, state, nav, toaster],
        move |_| async move {
            use rustter_domain::post::Caption;
            use rustter_endpoint::post::{
                endpoint::{NewPost, NewPostOk},
                types::{Image, ImageKind, NewPostOptions},
            };

            let request = {
                use rustter_domain::post::{Headline, Message};
                NewPost {
                    content: Image {
                        caption: {
                            let caption = &state.read().caption;
                            if caption.is_empty() {
                                None
                            } else {
                                Some(Caption::new(caption).unwrap())
                            }
                        },
                        kind: ImageKind::DataUrl(state.read().image.clone().unwrap()),
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
            ImageInput {
                state: state.clone(),
            },
            // ImagePreview {
            //
            // },
            CaptionInput {
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
