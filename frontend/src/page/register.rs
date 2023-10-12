#![allow(non_snake_case)]

use crate::elements::{KeyedNotifications, KeyedNotificationsBox};
use crate::util::ApiClient;
use crate::{fetch_json, maybe_class, prelude::*};
use dioxus::prelude::*;
use rustter_domain::UserFacingError;

pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
    form_errors: KeyedNotifications,
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
            form_errors: KeyedNotifications::default(),
        }
    }

    pub fn can_submit(&self) -> bool {
        !(self.form_errors.has_messages()
            || self.username.current().is_empty()
            || self.password.current().is_empty())
    }
}

#[inline_props]
pub fn UsernameInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "username",
                "Username"
            },
            input {
                id: "username",
                class: "input-field",
                name:"username",
                placeholder: "Username",
                value: "{state.current()}",
                oninput: move |ev| oninput.call(ev)
            }
        }
    })
}

#[inline_props]
pub fn PasswordInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "password",
                "Password"
            },
            input {
                id: "password",
                class: "input-field",
                name:"password",
                placeholder: "Password",
                value: "{state.current()}",
                r#type: "password",
                oninput: move |ev| oninput.call(ev)
            }
        }
    })
}

pub fn Register(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let page_state = PageState::new(cx);
    let page_state = use_ref(cx, || page_state);

    let form_onsubmit = async_handler!(&cx, [api_client, page_state], move |_| async move {
        use rustter_endpoint::user::endpoint::{CreateUser, CreateUserOk};
        let request_data = {
            use rustter_domain::{Password, Username};
            CreateUser {
                username: Username::new(
                    page_state.with(|state| state.username.current().to_string()),
                )
                .unwrap(),
                password: Password::new(
                    page_state.with(|state| state.password.current().to_string()),
                )
                .unwrap(),
            }
        };
        let response = fetch_json!(<CreateUserOk>, api_client, request_data);
        match response {
            Ok(res) => (),
            Err(e) => (),
        }
    });

    let username_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        if let Err(e) = rustter_domain::Username::new(&ev.value) {
            page_state.with_mut(|state| state.form_errors.set("bad-username", e.formatted_error()))
        } else {
            page_state.with_mut(|state| state.form_errors.remove("bad-username"))
        }
        page_state.with_mut(|state| state.username.set(ev.value.clone()))
    });

    let password_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        if let Err(e) = rustter_domain::Password::new(&ev.value) {
            page_state.with_mut(|state| state.form_errors.set("bad-password", e.formatted_error()))
        } else {
            page_state.with_mut(|state| state.form_errors.remove("bad-password"))
        }
        page_state.with_mut(|state| state.password.set(ev.value.clone()))
    });

    let submit_button_style = maybe_class!(
        "button-disabled",
        !page_state.with(|state| state.can_submit())
    );

    // above equivalent to:
    // let submit_button_style = match page_state.with(|state| state.can_submit()) {
    //     false => "button-disabled",
    //     true => "",
    // };

    cx.render(rsx! {
        div {
            class: "flex flex-col gap-5 m-5",
            UsernameInput {
                state: page_state.with(|state|state.username.clone()),
                oninput:username_oninput
            }

            PasswordInput {
                state: page_state.with(|state|state.password.clone()),
                oninput: password_oninput
            }

            KeyedNotificationsBox {
                legend: "Form Errors",
                notifications:page_state.clone().with(|state|state.form_errors.clone())
            }

            button {
                class: "btn  {submit_button_style}",
                onclick: form_onsubmit,
                disabled: !page_state.with(|state|state.can_submit()),
                "Signup"
            }
        }
    })
}
