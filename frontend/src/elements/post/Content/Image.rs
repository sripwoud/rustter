use dioxus::prelude::*;
use rustter_endpoint::post::types;

#[inline_props]
pub fn Image<'a>(cx: Scope<'a>, image: &'a types::Image) -> Element {
    let url = if let types::ImageKind::Url(url) = &image.kind {
        url
    } else {
        return cx.render(rsx! (
            p { "image not found"}
        ));
    };

    let Caption = image.caption.as_ref().map(|caption| {
        rsx! (
            figcaption {
                em {"{caption.as_ref()}"}
            }
        )
    });

    render! {
        figure {
            class: "flex flex-col gap-2",
            Caption,
            img { class:"w-full object-contain max-h-[50vh]", src: "{url}" }
        }
    }
}
