use crate::elements::use_toaster;
use crate::fetch_json;
use crate::prelude::ApiClient;
use dioxus::prelude::*;

pub fn TrendingPosts(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);
    to_owned![api_client, toaster];

    use_future(cx, (), |_| async move {
        use rustter_endpoint::post::endpoint::{TrendingPosts, TrendingPostsOk};
        toaster.write().info("Fetching posts", None);

        let response = fetch_json!(<TrendingPostsOk>, api_client, TrendingPosts);
        match response {
            Ok(_res) => (),
            Err(e) => {
                toaster
                    .write()
                    .error(format!("failed to fetch posts {e}"), None);
            }
        }
    });

    render! {
        div {
            "trending posts"
        }
    }
}
