#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use rustter_domain::ids::PostId;
use rustter_endpoint::LikeStatus;

#[inline_props]
pub fn LikeDislike(
    cx: Scope,
    post_id: PostId,
    like_status: LikeStatus,
    likes: i64,
    dislikes: i64,
) -> Element {
    let post_manager = use_post_manager(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let like_icon = match like_status {
        LikeStatus::Like => "/static/icons/icon-like-selected.svg",
        _ => "/static/icons/icon-like.svg",
    };

    let dislike_icon = match like_status {
        LikeStatus::Dislike => "/static/icons/icon-dislike-selected.svg",
        _ => "/static/icons/icon-dislike.svg",
    };

    let toggle = async_handler!(
        &cx,
        [api_client, post_manager, toaster, post_id],
        move |like_status| async move {
            use rustter_endpoint::{Reaction, ReactionOk};

            let like_status = {
                // post is already liked/disliked, user clicks like/dislike again
                if post_manager.read().get(&post_id).unwrap().like_status == like_status {
                    LikeStatus::NoReaction
                } else {
                    like_status
                }
            };

            let reaction = Reaction {
                post_id,
                like_status,
            };

            match post_json!(<ReactionOk>,api_client, reaction) {
                Ok(ReactionOk {
                    like_status,
                    likes,
                    dislikes,
                    ..
                }) => {
                    post_manager.write().update(post_id, |post| {
                        post.like_status = like_status;
                        post.likes = likes;
                        post.dislikes = dislikes;
                    });
                }
                Err(e) => toaster
                    .write()
                    .error(format!("Failed to react to post {}", e), None),
            }
        }
    );

    cx.render(rsx! {
        div {
            class: "cursor-pointer",
            onclick: move |_| toggle(LikeStatus::Like),
            img {
                class: "actionbar-icon",
                src: like_icon,
            },
            div {
                class:"text-center",
                "{likes}"
            }
        },
        div {
            class: "cursor-pointer",
           onclick: move |_| toggle(LikeStatus::Dislike),
            img {
                class: "actionbar-icon",
                src: dislike_icon,
            },
            div {
                class:"text-center",
                "{dislikes}"
            }
        }
    })
}
