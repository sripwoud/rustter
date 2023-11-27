#![allow(non_snake_case)]

mod AppBarImgButton;
use AppBarImgButton::AppBarImgButton;

use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;

pub const BUTTON_SELECTED: &str = "border-slate-600";

enum AppBarRoute {
    NewChatPost,
    NewImagePost,
    NewPollPost,
    GoBack,
}

#[derive(Props)]
pub struct AppBarProps<'a> {
    title: &'a str,
}

pub fn AppBar<'a>(cx: Scope<'a, AppBarProps<'a>>) -> Element {
    let nav = use_navigator(cx);
    let path = window().location().pathname().unwrap();
    let path = path.split('/').last().unwrap();

    let slug = path
        .chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect::<String>();

    let buttons = vec![
        (
            AppBarRoute::NewChatPost,
            "/static/icons/icon-messages.svg",
            "Chat",
            "Post a new chat message",
        ),
        (
            AppBarRoute::NewImagePost,
            "/static/icons/icon-image.svg",
            "Image",
            "Post a new image",
        ),
        (
            AppBarRoute::NewPollPost,
            "/static/icons/icon-poll.svg",
            "Poll",
            "Post a new poll",
        ),
        (
            AppBarRoute::GoBack,
            "/static/icons/icon-back.svg",
            "Back",
            "Go to previous page",
        ),
    ];

    cx.render(rsx! {
        div {
            class: "max-w-[var(--content-max-width)] h-[var(--appbar-height)] fixed top-0 left-0 right-0 mx-auto z-50 appbar-bg-color appbar-border-color",
            div {
                class: "flex flex-row gap-8 items-center w-full h-full pr-5",
                div {
                    class: "cursor-pointer",
                    onclick: move |_| {

                    },
                    img {
                        class: "profile-portrait",
                        src: "" // TODO
                    }
                },
                div {
                    class: "text-xl font-bold mr-auto text-white",
                    "{cx.props.title}"
                }

                buttons.iter().map(|(route, img, label, button_title)| {
                   let disabled = slug.as_str() == *label;
                    let nav = nav.clone();

                    let click_handler: Box<dyn Fn(Event<MouseData>)> = match route {
                            AppBarRoute::NewChatPost => Box::new(move |_| { nav.replace(Route::NewChatPost {}); }),
                            AppBarRoute::NewImagePost => Box::new(move |_| { nav.replace(Route::NewImagePost {}); }),
                            AppBarRoute::NewPollPost => Box::new(move |_| { nav.replace(Route::NewPollPost {}); }),
                            AppBarRoute::GoBack => Box::new(move |_| { nav.go_back(); }),
                    };

                    rsx!(
                        AppBarImgButton {
                            // append_class: appbar::BUTTON_SELECTED,
                            disabled: disabled,
                            label: label,
                            img: img,
                            click_handler: click_handler,
                            title: button_title,
                        }
                    )
    })
            }
        }
    })
}
