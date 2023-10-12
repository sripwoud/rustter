#![allow(non_snake_case)]
use crate::prelude::*;

pub mod home;
use home::Home;
pub mod login;
use login::Login;
pub mod register;
use register::Register;
pub mod not_found;
use not_found::PageNotFound;



/// An enum of all of the possible routes in the app.
// #[derive(Routable, Clone)]
// #[rustfmt::skip]
// pub enum Route {
//     #[route("/")]
//     // If the name of the component and variant are the same you can omit the component and props name
//     // If they are different you can specify them like this:
//     // #[route("/", ComponentName, PropsName)]
//     // Home {},
//     // #[redirect("/home", || Route::Home {})]
//
//     #[nest("/account")]
//         #[route("/login")]
//         Login {},
//         #[route("/register")]
//         Register {},
//     #[end_nest]
//
//     #[route("/:..route")]
//     PageNotFound {
//         route: Vec<String>,
//     },
// }

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[route("/account/login")]
    Login {},
    #[route("/account/register")]
    Register {},
}
