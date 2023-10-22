use super::{Header, use_post_manager};
use super::Content;
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use rustter_domain::ids::PostId;

#[inline_props]
pub fn PublicPostEntry(cx: Scope, post_id: PostId) -> Element {
    let post_manager = use_post_manager(cx);
    let nav = use_navigator(cx);

    let this_post = {
        let post = post_manager.read().get(*post_id).unwrap().clone();
        use_state(cx, || post)
    };

    cx.render(rsx! {
        div {
            key: "{this_post.id.to_string()}",
            class: "flex flex-col gap-2 mx-4",
            div { /* profile image*/ },
            div {
                class: "flex flex-col gap-3",
                Header { post: this_post },
                Content { post: this_post},
                p { this_post.author.handle.clone() }
                hr {},
            }
        }
    })
}
