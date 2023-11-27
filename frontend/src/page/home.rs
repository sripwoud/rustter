use crate::prelude::AppBar;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    render! {
        AppBar {title: "Home"}
        h1 { "Home" }
        p { "Welcome to the home page!" }
    }
}
