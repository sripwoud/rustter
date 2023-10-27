#![allow(non_snake_case)]

mod Bookmark;
mod Boost;
mod LikeDislike;

use Bookmark::Bookmark;
use Boost::Boost;
use LikeDislike::LikeDislike;

use crate::prelude::*;
use dioxus::prelude::*;
use rustter_domain::ids::PostId;

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
            }

        }
    })
}
