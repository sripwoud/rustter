#![allow(non_snake_case)]
use super::page::Route;
use crate::elements::post::PostManager;
use crate::elements::{ToastRoot, Toaster};
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use fermi::{use_init_atom_root, AtomRef};

pub static TOASTER: AtomRef<Toaster> = AtomRef(|_| Toaster::default());
pub static POST_MANAGER: AtomRef<PostManager> = AtomRef(|_| PostManager::default());
pub static LOCAL_PROFILE: AtomRef<LocalProfile> = AtomRef(|_| LocalProfile::default());

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    cx.render(rsx! {
        main {
            Router::<Route> { }
        }
        ToastRoot {
            toaster: use_toaster(cx)
        }
    })
}
