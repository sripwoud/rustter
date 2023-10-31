use dioxus::prelude::*;
use rustter_domain::ids::PostId;
use rustter_endpoint::post::types;

#[inline_props]
pub fn Chat<'a>(cx: Scope<'a>, post_id: PostId, content: &'a types::Chat) -> Element {
    let Headline = content.headline.as_ref().map(|headline| {
        rsx! {
            h3 { headline.as_ref() }
        }
    });

    cx.render(rsx! {
        div {
            Headline,
            p { content.message.as_ref()}
        }
    })
}
