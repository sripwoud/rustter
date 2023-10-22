#![allow(non_snake_case)]
mod home;
mod login;
mod new;
mod not_found;
mod post;

use crate::elements::NavBar;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use home::Home;
use login::Login;
use new::{
    post::{chat::NewChatPost, image::NewImagePost, poll::NewPollPost},
    user::Register,
};
use not_found::NotFound;
use post::trending::TrendingPosts;
use std::iter::Iterator;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")] // TODO: Add redirect to login if not logged in
    Home {},
    #[redirect("/home", || Route::Home {})]
    #[route("/register")]
    Register {}, // TODO: Add redirect to home if logged in
    #[route("/login")] // TODO: Add redirect to home if logged in
    Login {},
    #[route("/posts")]
    TrendingPosts {},
    #[nest("/post")]
        #[route("/chat")]
        NewChatPost {},
        #[route("/image")]
        NewImagePost {},
        #[route("/poll")]
        NewPollPost {},
    #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}
