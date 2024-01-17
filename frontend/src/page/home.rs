use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use log::info;

pub fn Home(cx: Scope) -> Element {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();
    let post_manager = use_post_manager(cx);
    let nav = use_navigator(cx);

    // TODO: write macro for this?

    {
        to_owned![api_client, toaster, post_manager];

        use_future(cx, (), |_| async move {
            use rustter_endpoint::post::endpoint::{HomePosts, HomePostsOk};
            toaster.write().info("Fetching home posts", None);

            let response = fetch_json!(<HomePostsOk>, api_client, HomePosts);
            match response {
                Ok(res) => post_manager.write().populate(res.0.into_iter()),
                Err(e) => {
                    info!("failed to fetch posts {e}");
                    toaster
                        .write()
                        .error(format!("failed to fetch posts {e}"), None);
                }
            };
        });
    }

    let Posts = {
        let posts = post_manager.read().all_to_public();

        if posts.is_empty() {
            let TrendingLink = rsx! {
                a {
                    class: "link",
                    onclick: move|_| {
                        nav.push(Route::TrendingPosts {});
                    },
                    "trending,"
                }
            };

            rsx! {
                div {
                    class: "flex flex-col text-center justify-center h-[calc(100vh_-_var(--navbar-height)_-_var(--appbar-height))]",
                    span {
                        "Check out what's " TrendingLink " and follow some users to get started."
                    }
                }
            }
        } else {
            rsx! { posts.into_iter() }
        }
    };

    render! {
        AppBar {title: "Home", buttons: vec![
        (
            AppBarRoute::LikedPosts,
            "/static/icons/icon-like.svg",
            "Liked",
            "Show liked posts",
        ),
        (
            AppBarRoute::BookmarkedPosts,
            "/static/icons/icon-bookmark-saved.svg",
            "Bookmarks",
            "Show bookmarks",
        )
    ]
            }
        div {
         class:"overflow-y-auto max-h-[calc(100vh-var(--navbar-height))]",
            Posts
        }
    }
}
