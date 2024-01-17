use crate::page::Route;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use dioxus_router::prelude::Navigator;
use rustter_domain::ids::UserId;
use rustter_endpoint::post::types::PublicPost;

pub fn view_profile_onclick(nav: &Navigator, user_id: UserId) -> impl FnMut(MouseEvent) + '_ {
    move |_: MouseEvent| {
        nav.push(Route::ViewProfile {
            user_id: user_id.to_string(),
        });
    }
}

#[inline_props]
pub fn ProfileImage<'a>(cx: SCope<'a>, post: &'a PublicPost) -> Element {
    let nav = use_navigator(cx);
    let author = &post.author;
    let src = &author
        .profile_image
        .as_ref()
        .map(|url| url.as_str())
        .unwrap_or_else(|| "");

    render! {
        div {
            img {
                class:"profile-portrait cursor-pointer",
                onclick: view_profile_onclick(nav, post.author.id),
                src: "{src}"
            }
        }
    }
}
