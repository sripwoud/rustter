use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Props)]
pub struct NavButtonProps<'a> {
    to: Option<Route>,
    img: &'a str,
    label: &'a str,
    onclick: EventHandler<'a, MouseEvent>,
    highlight: Option<bool>,
    children: Element<'a>,
}

pub fn NavButton<'a>(cx: Scope<'a, NavButtonProps<'a>>) -> Element {
    let selected_bgcolor = maybe_class!("bg-slate-500", matches!(cx.props.highlight, Some(true)));

    match cx.props.to.clone() {
        Some(route) => {
            cx.render(rsx! {
                button {
                    Link {
                        class: "cursor-pointer flex flex-col items-center justify-center {selected_bgcolor}",
                        onclick: move |ev| cx.props.onclick.call(ev),
                        to: route,
                        img {
                            class: "invert",
                            src: cx.props.img,
                            width: "25px",
                            height: "25px",
                        },
                        div {
                            class: "text-sm text-white",
                            cx.props.label
                        },
                    }
                }
                    &cx.props.children
            })
        }
        None => {
            cx.render(rsx! {
                button {
                    class: "cursor-pointer flex flex-col items-center justify-center {selected_bgcolor}",
                    onclick: move |ev| cx.props.onclick.call(ev),
                    img {
                        class: "invert",
                        src: cx.props.img,
                        width: "25px",
                        height: "25px",
                    },
                    div {
                        class: "text-sm text-white",
                        cx.props.label
                    },
                    &cx.props.children
                }
            })
        }
    }
}
