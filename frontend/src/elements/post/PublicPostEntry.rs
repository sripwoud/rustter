use super::{use_post_manager, Header};
use super::{ActionBar, Content};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use rustter_domain::ids::PostId;

#[inline_props]
pub fn PublicPostEntry(cx: Scope, post_id: PostId) -> Element {
    let post_manager = use_post_manager(cx);
    let _nav = use_navigator(cx);

    let this_post = {
        let post = post_manager.read().get(post_id).unwrap().clone();
        use_state(cx, || post)
    };

    cx.render(rsx! {
        div {
            key: "{this_post.id.to_string()}",
            class: "flex flex-col gap-2 mb-4",
            div { /* profile image*/ },
            div {
                class: "flex flex-col gap-3 mx-4",
                Header { post: this_post },
                Content { post: this_post},
                ActionBar { post_id: this_post.id },
                p { this_post.author.handle.clone() }
                hr {},
            }
        }
    })
}
