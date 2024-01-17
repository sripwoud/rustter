use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use log::info;

pub fn LikedPosts(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();
    let post_manager = use_post_manager(cx);

    // TODO: write macro for this?

    {
        to_owned![api_client, toaster, post_manager];

        use_future(cx, (), |_| async move {
            use rustter_endpoint::post::endpoint::{LikedPosts, LikedPostsOk};
            toaster.write().info("Fetching liked posts", None);

            post_manager.write().clear();

            let response = fetch_json!(<LikedPostsOk>, api_client, LikedPosts);
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

    // FIXME: duplicate (home and bookmarks page), refactor
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
                        "You don't have any liked posts yet. Check out what's " TrendingLink " and follow some users to get started."
                    }
                }
            }
        } else {
            rsx! { posts.into_iter() }
        }
    };

    render! {
        AppBar {title: "Liked", buttons: vec![
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
        ),
        (
            AppBarRoute::Home,
            "/static/icons/icon-home.svg",
            "Home",
            "Go to home page",
        )
    ]
            }
        div {
         class:"overflow-y-auto max-h-[calc(100vh-var(--navbar-height))]",
            Posts
        }
    }
}
