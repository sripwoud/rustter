#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub mod home;
use home::Home;
pub mod login;
use login::Login;
pub mod register;
use register::Register;
pub mod not_found;
mod trending;
use trending::Trending;
mod new_post;
use new_post::NewPost;

use not_found::NotFound;

use crate::elements::NavBar;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[redirect("/home", || Route::Home {})]
        #[nest("/account")]
            #[route("/login")]
            Login {},
            #[route("/register")]
            Register {},
        #[end_nest]
        #[nest("/post")]
            #[route("/new")]
            NewPost {},
            #[route("/trending")]
            Trending {},
        #[end_nest]
    #[end_layout]

    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}
