#![allow(non_snake_case)]

use crate::elements::navbar::NewPostPopupButton::NewPostPopupButton;
use crate::prelude::*;
use dioxus::prelude::*;

#[inline_props]
pub fn NewPostPopup(cx: Scope, hide: UseState<bool>) -> Element {
    let hide_class = maybe_class!("hidden", *hide.get());

    cx.render(rsx! {
        div {
            class: "flex flex-col absolute right-0 bottom-[var(--navbar-height)] w-28 items-center navbar-bg-color text-white text-sm {hide_class}",
            NewPostPopupButton {
                to: Route::NewChatPost {},
                img: "/static/icons/icon-messages.svg",
                label: "Chat",
                onclick:move|_|{hide.set(true);}
            },
            NewPostPopupButton {
                to: Route::NewPollPost {},
                img: "/static/icons/icon-poll.svg",
                label: "Poll",
                onclick:move|_|{hide.set(true);}
            },
            NewPostPopupButton {
                to: Route::NewImagePost {},
                img: "/static/icons/icon-image.svg",
                label: "Image",
                onclick:move|_|{hide.set(true);}
            },
        }
    })
}
