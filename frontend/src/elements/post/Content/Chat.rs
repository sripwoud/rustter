use dioxus::prelude::*;
use rustter_domain::ids::PostId;
use rustter_endpoint::post::types;

#[inline_props]
pub fn Chat<'a>(cx: Scope<'a>, _post_id: Option<PostId>, chat: &'a types::Chat) -> Element {
    let Headline = chat.headline.as_ref().map(|headline| {
        rsx! {
            h3 { headline.as_ref() }
        }
    });

    cx.render(rsx! {
        div {
            Headline,
            p { chat.message.as_ref()}
        }
    })
}
