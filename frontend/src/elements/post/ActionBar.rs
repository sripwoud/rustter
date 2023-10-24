#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use log::info;
use rustter_domain::ids::PostId;

#[inline_props]
pub fn Bookmark(cx: Scope, post_id: PostId, bookmarked: bool) -> Element {
    let post_manager = use_post_manager(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let handle_click = async_handler!(
        &cx,
        [api_client, post_manager, toaster, post_id],
        move |_| async move {
            use rustter_endpoint::post::{
                endpoint::{Bookmark, BookmarkOk},
                types::BookmarkAction,
            };

            let action = match post_manager.read().get(&post_id).unwrap().bookmarked {
                false => BookmarkAction::Save,
                true => BookmarkAction::Remove,
            };

            let bookmark = Bookmark { post_id, action };
            info!("{:?}", bookmark);
            match post_json!(<BookmarkOk>,api_client, bookmark) {
                Ok(BookmarkOk { status }) => {
                    post_manager.write().update(post_id, |post| {
                        post.bookmarked = status.into();
                    });
                }
                Err(e) => toaster
                    .write()
                    .error(format!("Failed to bookmark post {}", e), None),
            }
        }
    );

    let src = match bookmarked {
        true => "/static/icons/icon-bookmark-saved.svg",
        false => "/static/icons/icon-bookmark.svg",
    };

    cx.render(rsx! {
        div {
            class: "cursor-pointer",
            onclick: handle_click,
            img {
                class: "actionbar-icon",
                src: src,
            }
        }
    })
}

#[inline_props]
pub fn ActionBar(cx: Scope, post_id: PostId) -> Element {
    let post_manager = use_post_manager(cx);
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
            }
            // like, dislike
            // comment

        }
    })
}
