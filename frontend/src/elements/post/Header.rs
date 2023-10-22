use dioxus::prelude::*;
use rustter_endpoint::post::types::PublicPost;

#[inline_props]
pub fn Header<'a>(cx: SCope<'a>, post: &'a PublicPost) -> Element {
    let (post_date, post_time) = {
        let date = post.time_posted.format("%Y-%m-%d");
        let time = post.time_posted.format("%H:%M:%S");
        (date, time)
    };

    let display_name = match &post.author.display_name {
        Some(name) => name.as_ref(),
        None => "",
    };
    let handle = &post.author.handle;

    cx.render(rsx! {
        div {
            class: "flex flex-row justify-between",
            div {
                class: "cursor-pointer",
                onclick: move|_|(),
                div {"{display_name}"},
                div {
                    class:"font-light",
                    "{handle}"
                },
            }
                div {
                    class:"text-right",
                    div{"{post_date}"},
                    div{"{post_time}"}
                }
        }
    })
}
