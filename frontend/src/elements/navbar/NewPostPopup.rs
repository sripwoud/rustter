#![allow(non_snake_case)]

use crate::elements::navbar::NewPostPopupButton::NewPostPopupButton;
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[inline_props]
pub fn NewPostPopup(cx: Scope, hide: UseState<bool>) -> Element {
    let hide_class = maybe_class!("hidden", *hide.get());
    let nav = use_navigator(cx);
    const BUTTON_CLASS: &str = "grid grid-cols-[20px_1fr] gap-4 pl-4 justify-center items-center w-full h-12 border-y navbar-border-color";

    cx.render(rsx! {
        div {
            class: "flex flex-col absolute right-0 bottom-[var(--navbar-height)] w-28 items-center navbar-bg-color text-white text-sm {hide_class}",
            NewPostPopupButton {
                to: Route::NewChatPost {},
                img: "/static/icons/icon-messages.svg",
                label: "Text",
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
