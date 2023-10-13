#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
mod home;
use home::Home;
mod login;
use login::Login;
mod new;
use new::{
    post::{chat::NewChatPost, image::NewImagePost, poll::NewPollPost},
    user::Register,
};
mod not_found;
use not_found::NotFound;
mod post;
use post::trending::TrendingPosts;

use crate::elements::NavBar;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[redirect("/home", || Route::Home {})]
    #[route("/login")]
    Login {},
    #[nest("/new")]
        #[route("/user")]
        Register {},
        #[nest("/post")]
            #[route("/chat")]
            NewChatPost {},
            #[route("/image")]
            NewImagePost {},
            #[route("/poll")]
            NewPollPost {},
        #[end_nest]
    #[end_nest]
        #[nest("/post")]
          #[route("/trending")]
          TrendingPosts {},
        #[end_nest]
    #[end_layout]

    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}
