use dioxus::prelude::*;
use rustter_endpoint::post::types;

mod Chat;
use Chat::Chat;
mod Image;
use Image::Image;

#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, post: &'a types::PublicPost) -> Element {
    cx.render(rsx! {
        div {
            match &post.content {
                types::Content::Chat(chat) => rsx! {
                    Chat { content: chat }
                },
                types::Content::Poll(_poll) => rsx! {
                    p { "todo: implement poll component" }
                },
                types::Content::Image(image)  => rsx! { Image { image: image } }
            }
        }
    })
}
