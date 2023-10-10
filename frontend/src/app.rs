#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::use_init_atom_root;
use rustter_endpoint::Endpoint;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let _other_things = rsx! { p { "hi"} };
    cx.render(rsx! {
        Router {
            Route { to: rustter_endpoint::CreateUser::URL, page::Register { } },
            Route { to: rustter_endpoint::Login::URL, page::Login {} }

        }
    })
}
