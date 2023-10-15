#![allow(non_snake_case)]
use crate::page::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::{AtomRef, use_init_atom_root};
use crate::elements::Toaster;

pub static TOASTER: AtomRef<Toaster> = AtomRef(|_| Toaster::new());
pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let _other_things = rsx! { p { "hi"} };
    cx.render(rsx! {
        Router::<Route> {}
    })
}
