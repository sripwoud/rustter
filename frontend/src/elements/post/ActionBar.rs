#![allow(non_snake_case)]

mod Bookmark;
mod Boost;
mod LikeDislike;
mod QuickRespond;

use Bookmark::Bookmark;
use Boost::Boost;
use LikeDislike::LikeDislike;
use QuickRespond::QuickRespond;

use crate::prelude::*;
use dioxus::prelude::*;
use rustter_domain::ids::PostId;

#[inline_props]
pub fn Comment(cx: Scope, opened: UseState<bool>) -> Element {
    let on_click = sync_handler!([opened], move |_| {
        let currrent = *opened.get();
        opened.set(!currrent);
    });

    cx.render(rsx! {
            div {
                class:"cursor-pointer",
                onclick: on_click,
                img {
                    class:"actionbar-icon",
                    src:"/static/icons/icon-messages.svg",
                }
            }
    })
}

#[inline_props]
pub fn QuickRespondBox(cx: Scope, opened: UseState<bool>) -> Element {
    let element = match *opened.get() {
        true => {
            to_owned![opened];
            Some(rsx! {
                QuickRespond {
                    opened: opened,
                }
            })
        }
        false => None,
    };

    cx.render(rsx! {element})
}

#[inline_props]
pub fn ActionBar(cx: Scope, post_id: PostId) -> Element {
    let post_manager = use_post_manager(cx);
    let opened = use_state(cx, || false);
    let post = post_manager.read();
    let post = post.get(post_id).unwrap();
    // let post_id = post.id;

    cx.render(rsx! {
        div {
            class: "flex flex-row justify-between w-full opacity-70 mt-4",
            // boost
            Bookmark {
                bookmarked: post.bookmarked,
                post_id: post.id,
            },
            LikeDislike {
                post_id: post.id,
                like_status: post.like_status,
                likes: post.likes,
                dislikes: post.dislikes,
            },
           Boost {
                post_id: post.id,
                boosted: post.boosted,
                boosts: post.boosts,
            },
            Comment {
                opened: opened.clone(),
            },

        },
        QuickRespondBox {
            opened: opened.clone(),
        }
    })
}
