use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Home" }
        p { "Welcome to the home page!" }
    }
}
