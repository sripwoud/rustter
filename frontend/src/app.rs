#![allow(non_snake_case)]
use crate::elements::{use_toaster, ToastRoot, Toaster};
use crate::page::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::{use_init_atom_root, AtomRef};

pub static TOASTER: AtomRef<Toaster> = AtomRef(|_| Toaster::default());

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    cx.render(rsx! {
        Router::<Route> { },
        ToastRoot {
            toaster: use_toaster(cx)
        }
    })
}
