#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::debug;
#[derive(Props)]
pub struct AppBarImgButtonProps<'a, F>
where
    F: Fn(Event<MouseData>),
{
    append_class: Option<&'a str>,
    disabled: Option<bool>,
    label: &'a str,
    img: &'a str,
    click_handler: Option<F>,
    title: Option<&'a str>,
}

pub fn AppBarImgButton<'a, F>(cx: Scope<'a, AppBarImgButtonProps<'a, F>>) -> Element
where
    F: Fn(Event<MouseData>),
{
    let append_class = cx.props.append_class.unwrap_or_default();

    cx.render(rsx! {
        button {
            class: "flex flex-col w-10 h-14 justify-center items-center hover:bg-gray-200 focus:outline-none focus:bg-gray-200 active:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed",
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
