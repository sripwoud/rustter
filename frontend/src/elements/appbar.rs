#![allow(non_snake_case)]

pub mod AppBarImgButton;

use dioxus::prelude::*;

pub const BUTTON_SELECTED: &str = "border-slate-600";

#[derive(Props)]
pub struct AppBarProps<'a> {
    children: Element<'a>,
    title: &'a str,
}

pub fn AppBar<'a>(cx: Scope<'a, AppBarProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "max-w-[var(--content-max-width)] h-[var(--appbar-height)] fixed top-0 left-0 right-0 mx-auto z-50 appbar-bg-color appbar-border-color",
            div {
                class: "flex flex-row gap-8 items-center w-full h-full pr-5",
                div {
                    class: "cursor-pointer",
                    onclick: move |_| (),
                    img {
                        class: "profile-portrait",
                        src: "" // TODO
                    }
                },
                div {
                    class: "text-xl font-bold mr-auto text-white",
                    "{cx.props.title}"
                }
                &cx.props.children
            }
        }
    })
}
