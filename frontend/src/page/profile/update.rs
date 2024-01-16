#![allow(non_snake_case)]

use crate::elements::{KeyedNotifications, KeyedNotificationsBox};
use crate::prelude::*;
use crate::util::document;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use log::info;
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
            class: "flex flex-row justify-center mt-[10px]",
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
                    if !&ev.value.is_empty() {
                        match Email::new(&ev.value) {
                           Ok(_) => {
                                state.with_mut(|state|state.form_errors.remove("bad-email"));
                            },
                            Err(e) => {
                                state.with_mut(|state|state.form_errors.set("bad-email", e.formatted_error()));
                            }
                        }
                    } else {
                        state.with_mut(|state|state.form_errors.remove("bad-email"));
                    }
                    state.with_mut(|state| state.email = ev.value.clone());
                }
            }
        }
    }
}

#[inline_props]
pub fn PasswordInput(cx: Scope, state: UseRef<PageState>) -> Element {
    use rustter_domain::user::Password;

    let check_passwords_mismatch =
        move || match state.with(|state| state.password == state.password_confirmation) {
            true => {
                state.with_mut(|state| state.form_errors.remove("password-mismatch"));
            }
            false => {
                state.with_mut(|state| {
                    state
                        .form_errors
                        .set("password-mismatch", "Passwords don't match")
                });
            }
        };

    render! {
    fieldset {
        class:"fieldset flex flex-row justify-around",
        legend { "Set new password"}
        div {
            class:"flex flex-col",
            label {
                r#for:"password",
                "Password",
            },
            input {
                class:"input-field",
                r#type: "password",
                id:"password",
                value:"{state.read().password}",
                oninput:move|ev|{
                    match Password::new(&ev.value) {
                       Ok(_) => {
                            state.with_mut(|state|state.form_errors.remove("bad-password"));
                        },
                        Err(e) => {
                            state.with_mut(|state|state.form_errors.set("bad-password", e.formatted_error()));
                        }
                    }
                    state.with_mut(|state| state.password = ev.value.clone());
                    state.with_mut(|state| state.password_confirmation ="".to_string());

                    if state.with(|state| state.password.is_empty()) {
                        state.with_mut(|state|state.form_errors.remove("bad-password"));
                        state.with_mut(|state|state.form_errors.remove("password-mismatch"));
                    } else {
                            check_passwords_mismatch();
                    }
                }
            }
        }
        div {
            class:"flex flex-col",
            label {
                r#for:"password-confirm",
                "Confirm",
            },
            input {
                class:"input-field",
                r#type: "password",
                id:"password-confirm",
                value:"{state.read().password_confirmation}",
                oninput:move|ev|{
                    state.with_mut(|state| state.password_confirmation = ev.value.clone());
                    check_passwords_mismatch();
                }
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
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);

    // fetch profile
    {
        to_owned![api_client, toaster, page_state];

        use_future(cx, (), |_| async move {
            use rustter_endpoint::user::endpoint::{GetMyProfile, GetMyProfileOk};
            toaster.write().info("Fetching profile", None);

            let response = fetch_json!(<GetMyProfileOk>, api_client, GetMyProfile);
            match response {
                Ok(res) => page_state.with_mut(|state| {
                    state.display_name = res.display_name.unwrap_or_default();
                    state.email = res.email.unwrap_or_default();
                    state.profile_image = res
                        .profile_image
                        .map(|img| PreviewImageData::Remote(img.to_string()))
                }),
                Err(e) => {
                    info!("failed to fetch profile {e}");
                    toaster
                        .write()
                        .error(format!("failed to fetch posts {e}"), None);
                }
            };
        });
    }

    let disable_submit = page_state.with(|state| state.form_errors.has_messages());
    let submit_btn_style = maybe_class!("button-disabled", disable_submit);

    let form_onsubmit = async_handler!(
        &cx,
        [api_client, page_state, nav, toaster],
        move |_| async move {
            use rustter_endpoint::user::endpoint::{
                Update, UpdateProfile as UpdateProfilePayload, UpdateProfileOk,
            };
            let request_data = {
                use rustter_domain::Password;
                UpdateProfilePayload {
                    display_name: {
                        let name = page_state.with(|state| state.display_name.clone());
                        if name.is_empty() {
                            Update::SetNull
                        } else {
                            Update::Change(name)
                        }
                    },
                    email: {
                        let email = page_state.with(|state| state.email.clone());
                        if email.is_empty() {
                            Update::SetNull
                        } else {
                            Update::Change(email)
                        }
                    },
                    password: {
                        let password = page_state.with(|state| state.password.clone());
                        if password.is_empty() {
                            Update::NoChange
                        } else {
                            Update::Change(Password::new(password).unwrap())
                        }
                    },
                    profile_image: {
                        let profile_image = page_state.with(|state| state.profile_image.clone());
                        match profile_image {
                            Some(PreviewImageData::DataUrl(data)) => Update::Change(data),
                            Some(PreviewImageData::Remote(_)) => Update::NoChange,
                            None => Update::SetNull,
                        }
                    },
                }
            };

            let response = post_json!(<UpdateProfileOk>, api_client, request_data);
            match response {
                Ok(_res) => {
                    toaster.write().success("Profile updated", None);
                    nav.push(Route::Home {});
                }
                Err(e) => {
                    toaster
                        .write()
                        .error(format!("Failed to update profile: {}", e), None);
                }
            }
        }
    );

    cx.render(rsx! {
        AppBar {
            title: "Update profile",
            buttons: vec![
                (
            AppBarRoute::GoBack,
            "/static/icons/icon-back.svg",
            "Back",
            "Go to previous page",
        ),
            ]
        }
        div {
            class: "flex flex-col w-full gap-3 mt-[var(--appbar-height)]",
            ImagePreview {state: page_state.clone()},
            ImageInput { state:page_state.clone()},
            DisplayNameInput {state: page_state.clone()},
            EmailInput {state: page_state.clone()},
            PasswordInput { state: page_state.clone()}
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
                    class:"btn {submit_btn_style}",
                    disabled: disable_submit,
                    onclick: form_onsubmit,
                    "Submit"
                }
            }
        }
    })
}
