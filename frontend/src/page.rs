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
use not_found::NotFound;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[redirect("/home", || Route::Home {})]

    #[nest("/account")]
        #[route("/login")]
        Login {},
        #[route("/register")]
        Register {},
    #[end_nest]

    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}
