#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct AppBarImgButtonProps<'a, F> where F: Fn(Event<MouseData>) {
    append_class: Option<&'a str>,
    disabled: Option<bool>,
    label: &'a str,
    img: &'a str,
    onclick: Option<F>,
    title: Option<&'a str>,
}

pub fn AppBarImgButton<'a, F>(cx: Scope<'a, AppBarImgButtonProps<'a, impl Fn(Event<MouseData>)>>) -> Element<'a> {
    let append_class = cx.props.append_class.unwrap_or_default();

    cx.render(rsx! {
        button {
            class: "flex flex-col w-10 h-14 justify-end items-center border-slate-200 border-b-4 {append_class}",
            disabled: cx.props.disabled.unwrap_or_default(),
            onclick: |ev| {
                if let Some(onclick) = &cx.props.onclick {
                    onclick(ev)
                }
            },
            title: cx.props.title,
            img {
                class: "w-6 h-6",
                src: cx.props.img,
                width: "25px",
                height: "25px",
            },
            span {
                class: "text-sm",
                cx.props.label
            },
        }
    })
}
