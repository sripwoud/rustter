use super::Chat;
use dioxus::prelude::*;
use rustter_endpoint::post::types;
#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, post: &'a types::PublicPost) -> Element {
    cx.render(rsx! {
        div {
            match &post.content {
                types::Content::Chat(chat) => rsx! {
                    Chat { post_id:post.id, content: chat }
                },
                types::Content::Poll(poll) => rsx! {
                    p { "todo: implement poll component" }
                },
                types::Content::Image(image) => rsx! {
                    // img { src: image.src.as_str() }
                    p {"todo: implement image component"}
                },
            }
        }
    })
}
