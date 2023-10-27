#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use rustter_domain::ids::PostId;

#[inline_props]
pub fn Boost(cx: Scope, post_id: PostId, boosted: bool, boosts: i64) -> Element {
    let post_manager = use_post_manager(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let src = match boosted {
        true => "/static/icons/icon-boosted.svg",
        false => "/static/icons/icon-boost.svg",
    };

    let handle_click = async_handler!(
        &cx,
        [api_client, post_manager, toaster, post_id],
        move |_| async move {
            use rustter_endpoint::post::{
                endpoint::{Boost, BoostOk},
                types::BoostAction,
            };

            let action = match post_manager.read().get(&post_id).unwrap().boosted {
                false => BoostAction::Add,
                true => BoostAction::Remove,
            };

            let boost = Boost { post_id, action };

            match post_json!(<BoostOk>,api_client, boost) {
                Ok(BoostOk { status }) => {
                    post_manager.write().update(post_id, |post| {
                        post.boosted = status.into();
                        if post.boosted {
                            post.boosts += 1;
                        } else {
                            post.boosts -= 1;
                        }
                    });
                }
                Err(e) => toaster
                    .write()
                    .error(format!("Failed to boost post {}", e), None),
            }
        }
    );

    cx.render(rsx! {
        div {
            class: "cursor-pointer",
            onclick: handle_click,
            img {
                class: "actionbar-icon",
                src: src,
            },
            div {
                class:"text-center",
                "{boosts}"
            }
        }
    })
}
