#![allow(non_snake_case)]

use super::{NavButton, NewPostPopup};
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

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
