use crate::elements::use_toaster;
use crate::elements::PublicPostEntry;
use crate::prelude::*;
use dioxus::prelude::*;
use log::info;

pub fn TrendingPosts(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);
    let post_manager = use_post_manager(cx);

    {
        to_owned![api_client, toaster, post_manager];

        use_future(cx, (), |_| async move {
            use rustter_endpoint::post::endpoint::{TrendingPosts, TrendingPostsOk};
            toaster.write().info("Fetching posts", None);

            let response = fetch_json!(<TrendingPostsOk>, api_client, TrendingPosts);
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

    let TrendingPosts = post_manager
        .read()
        .posts
        .iter()
        .map(|(&id, _)| {
            rsx! { PublicPostEntry { post_id: id } }
        })
        .collect::<Vec<LazyNodes>>();

    cx.render(rsx! {
        div {
         class:"overflow-y-auto max-h-[calc(100vh-var(--navbar-height))]",
            TrendingPosts.into_iter()
        }
    })
}
