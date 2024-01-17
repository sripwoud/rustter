#![allow(non_snake_case)]
mod home;
mod login;
mod new;
mod not_found;
mod posts;
mod profile;
pub use profile::{update::UpdateProfile, view::ViewProfile};

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
use posts::{bookmarked::BookmarkedPosts, liked::LikedPosts, trending::TrendingPosts};

use std::iter::Iterator;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[redirect("/home", || Route::Home {})]
    #[route("/register")]
    Register {},
    #[route("/login")]
    Login {},
    #[nest("/posts")]
        #[route("/trending")]
        TrendingPosts {},
        #[route("/liked")]
        LikedPosts {},
        #[route("/bookmarks")]
        BookmarkedPosts {},
    #[end_nest]
    #[nest("/post")]
        #[route("/chat")]
        NewChatPost {},
        #[route("/image")]
        NewImagePost {},
        #[route("/poll")]
        NewPollPost {},
    #[end_nest]
    #[nest("/profile")]
        #[route("/update")]
        UpdateProfile {},
        #[route("/view/:user_id")]
        ViewProfile {
            user_id: String
        },
    #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}
