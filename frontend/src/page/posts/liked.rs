use crate::prelude::*;
use dioxus::prelude::*;
use log::info;

pub fn LikedPosts(cx: Scope) -> Element {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();
    let post_manager = use_post_manager(cx);

    // TODO: write macro for this?

    {
        to_owned![api_client, toaster, post_manager];

        use_future(cx, (), |_| async move {
            use rustter_endpoint::post::endpoint::{LikedPosts, LikedPostsOk};
            toaster.write().info("Fetching liked posts", None);

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

    let Posts = post_manager.read().all_to_public();

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
            Posts.into_iter()
        }
    }
}
