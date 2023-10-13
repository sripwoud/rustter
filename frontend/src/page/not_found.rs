#![allow(unused_variables)]
use dioxus::prelude::*;

#[inline_props]
pub fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        // p { "We are terribly sorry, but the page you requested doesn't exist." }
        // pre {
        //     color: "red",
        //     "log:\nattempted to navigate to: {route:?}"
        // }
    }
}
