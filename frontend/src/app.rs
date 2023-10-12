#![allow(non_snake_case)]
use fermi::use_init_atom_root;
use crate::page::Route;
use crate::prelude::*;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    // let _other_things = rsx! { p { "hi"} };
    render! {
        Router::<Route> {}
    }
}
