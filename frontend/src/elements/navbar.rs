#![allow(non_snake_case)]

use crate::page::Route;
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[inline_props]
pub fn NewPostPopup(cx: Scope, hide: UseState<bool>) -> Element {
    let hide_class = maybe_class!("hidden", *hide.get());
    let nav = use_navigator(cx);
    const BUTTON_CLASS: &str = "grid grid-cols-[20px_1fr] gap-4 pl-4 justify-center items-center w-full h-12 border-y navbar-border-color";

    cx.render(rsx! {
        div {
            class: "flex flex-col absolute right-0 bottom-[var(--navbar-height)] w-28 items-center navbar-bg-color text-white text-sm {hide_class}",
            div {
                class: BUTTON_CLASS,
                onclick: move |_| {
                    hide.set(true);
                    nav.push(Route::NewPollPost {});
                },
                img {
                    class: "invert",
                    src: "/static/icons/icon-poll.svg",
                },
                "Poll"
            },
            div {
                class: BUTTON_CLASS,
                onclick: move |_| {
                    hide.set(true);
                    nav.push(Route::NewImagePost {});
                },
                img {
                    class: "invert",
                    src: "/static/icons/icon-image.svg",
                },
                "Image"
            },
            div {
                class: BUTTON_CLASS,
                onclick: move |_| {
                    hide.set(true);
                    nav.push(Route::NewChatPost {});
                },
                img {
                    class: "invert",
                    src: "/static/icons/icon-messages.svg",
                },
                "Chat"
            }
        }
    })
}

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

pub fn NavBar(cx: Scope) -> Element {
    let new_post_popup_is_hidden = use_state(cx, || true);
    let hide_new_post_popup = move |_| {
        let is_hidden = *new_post_popup_is_hidden.get();
        new_post_popup_is_hidden.set(!is_hidden)
    };

    cx.render(rsx! {
        nav {
            class: "max-w-[var(--content-max-width)] h-[var(--navbar-height)] fixed bottom-0 left-0 right-0 mx-auto border-t navbar-bg-color navbar-border-color",
            div {
                class: "grid grid-cols-3 justify-around w-full h-full items-center shadow-inner",
                // NavButton {
                //     img: "/static/icons/icon-search.svg",
                //     label: "Search",
                //     onclick: |_| (),
                //     to: Route::Trending {}
                // },
                NavButton {
                    img: "/static/icons/icon-trending.svg",
                    label: "Trending",
                    onclick: |_|{},
                    to: Route::TrendingPosts {}
                },
                NavButton {
                    img: "/static/icons/icon-home.svg",
                    label: "Home",
                    onclick: |_|{},
                    to: Route::Home {}
                },
                // NavButton {
                //     img: "/static/icons/icon-messages.svg",
                //     label: "DM",
                //     onclick: |_| (),
                //     to: Route::Home {}
                // },
                NavButton {
                    img: "/static/icons/icon-new-post.svg",
                    label: "Post",
                    onclick: hide_new_post_popup,
                    NewPostPopup {
                        hide: new_post_popup_is_hidden.clone()
                    }
                }
            }
        }
        Outlet::<Route> {}
    })
}
