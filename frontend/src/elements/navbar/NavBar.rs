#![allow(non_snake_case)]

use super::{NavButton, NewPostPopup};
use crate::elements::Sidebar;
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use rustter_endpoint::{CreateUser, Endpoint, Login};

pub fn Init(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let nav = use_navigator(cx);
    let toaster = use_toaster(cx);
    let local_profile = use_local_profile(cx);

    // fetch local profile
    {
        to_owned![api_client, toaster, nav, local_profile];

        use_future(cx, (), |_| async move {
            use rustter_endpoint::user::endpoint::{GetMyProfile, GetMyProfileOk};

            let response = fetch_json!(<GetMyProfileOk>, api_client, GetMyProfile);
            match response {
                Ok(res) => {
                    local_profile.write().image = res.profile_image;
                    local_profile.write().user_id = Some(res.user_id);
                }
                Err(e) => {
                    toaster.write().error(
                        format!("Please login or create an account first: {e}"),
                        None,
                    );
                    nav.push(Route::Login {});
                }
            }
        });
    }

    None
}

pub fn NavBar(cx: Scope) -> Element {
    let current_route = window().location().pathname().unwrap();
    // let navigator = use_navigator(cx);
    let hide_navbar = use_state(cx, || false);
    let new_post_popup_is_hidden = use_state(cx, || true);
    let hide_new_post_popup = move |_| {
        let is_hidden = *new_post_popup_is_hidden.get();
        new_post_popup_is_hidden.set(!is_hidden)
    };

    use_effect(cx, (&current_route,), |(current_route,)| {
        to_owned![hide_navbar];
        async move {
            let should_hide = current_route == Login::URL || current_route == CreateUser::URL;
            hide_navbar.set(should_hide);
        }
    });

    let hide_navbar_class = maybe_class!("hidden", *hide_navbar.get());

    cx.render(rsx! {
        Init {},
        Sidebar {},
        nav {
            class: "max-w-[var(--content-max-width)] h-[var(--navbar-height)] fixed bottom-0 left-0 right-0 mx-auto border-t navbar-bg-color navbar-border-color {hide_navbar_class}",
            div {
                class: "grid grid-cols-3 justify-around w-full h-full items-center shadow-inner",
                NavButton {
                    img: "/static/icons/icon-home.svg",
                    label: "Home",
                    onclick: |_|{},
                    to: Route::Home {}
                },
                // NavButton {
                //     img: "/static/icons/icon-search.svg",
                //     label: "Search",
                //     onclick: |_| (),
                //     to: Route::Trending {}
                // },
                NavButton {
                    img: "/static/icons/icon-trending.svg",
                    label: "Trending",
                    onclick: |_|{},
                    to: Route::TrendingPosts {}
                },
                // NavButton {
                //     img: "/static/icons/icon-messages.svg",
                //     label: "DM",
                //     onclick: |_| (),
                //     to: Route::Home {}
                // },
                NavButton {
                    img: "/static/icons/icon-new-post.svg",
                    label: "Post",
                    onclick: hide_new_post_popup,
                    NewPostPopup {
                        hide: new_post_popup_is_hidden.clone()
                    }
                }
            }
        },
        Outlet::<Route> {}
    })
}
