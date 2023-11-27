#![allow(non_snake_case)]
use crate::elements::post::PostManager;
use crate::elements::{use_toaster, ToastRoot, Toaster};
use crate::page::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::{use_init_atom_root, AtomRef};

pub static TOASTER: AtomRef<Toaster> = AtomRef(|_| Toaster::default());
pub static POST_MANAGER: AtomRef<PostManager> = AtomRef(|_| PostManager::default());

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    cx.render(rsx! {
        main{
            class:"max-w-[var(--content-max-width)] mt-[var(--appbar-height)] min-w-[var(--content-min-width)] mb-[var(--navbar-height)] mx-auto p-4",
            Router::<Route> { }
        }
        ToastRoot {
            toaster: use_toaster(cx)
        }
    })
}
