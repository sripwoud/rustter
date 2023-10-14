use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Props)]
pub struct NewPostPopupButtonProps<'a> {
    to: Route,
    img: &'a str,
    label: &'a str,
    onclick: EventHandler<'a, MouseEvent>,
}

pub fn NewPostPopupButton<'a>(cx: Scope<'a, NewPostPopupButtonProps<'a>>) -> Element {
    const BUTTON_CLASS: &str = "grid grid-cols-[20px_1fr] gap-4 pl-4 justify-center items-center w-full h-12 border-y navbar-border-color";

    cx.render(rsx! {
        Link {
            to: cx.props.to.clone(),
            class: BUTTON_CLASS,
            onclick: move |ev| cx.props.onclick.call(ev),
            img {
                class: "invert",
                src:cx.props.img,
            },
            cx.props.label
        }
    })
}
