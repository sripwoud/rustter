use dioxus::prelude::*;
use rustter_endpoint::post::types;

mod Chat;
use Chat::Chat;
mod Image;
mod Poll;
use Poll::Poll;

use Image::Image;

#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, post: &'a types::PublicPost) -> Element {
    cx.render(rsx! {
        div {
            match &post.content {
                types::Content::Chat(chat) => rsx! {
                    Chat { chat: chat }
                },
                types::Content::Poll(poll) => rsx! {
                       Poll { poll: poll, post_id: post.id }
                },
                types::Content::Image(image)  => rsx! { Image { image: image } }
            }
        }
    })
}
