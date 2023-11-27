#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct AppBarImgButtonProps<'a> {
    // pub append_class: Option<&'a str>,
    pub disabled: Option<bool>,
    pub label: &'a str,
    pub img: &'a str,
    pub click_handler: Option<Box<dyn Fn(Event<MouseData>)>>,
    pub title: Option<&'a str>,
}

pub fn AppBarImgButton<'a>(cx: Scope<'a, AppBarImgButtonProps<'a>>) -> Element {
    // let append_class = cx.props.append_class.unwrap_or_default();

    cx.render(rsx! {
        button {
            class: "flex flex-col w-10 h-14 justify-center items-center hover:bg-indigo-300 disabled:opacity-50 disabled:cursor-not-allowed",
            disabled: cx.props.disabled.unwrap_or_default(),
            onclick: |ev| {
                if let Some(onclick) = &cx.props.click_handler {
                    onclick(ev)
                }
            },
            title: cx.props.title,
            img {
                class: "w-6 h-6 invert",
                src: cx.props.img,
            },
            span {
                class: "text-sm text-white",
                "{cx.props.label}"
            },
        }
    })
}
