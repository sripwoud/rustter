#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use rustter_domain::ids::UserId;
use std::str::FromStr;

#[inline_props]
pub fn ViewProfile(cx: Scope, user_id: String) -> Element {
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);
    let post_manager = use_post_manager(cx);
    let _local_profile = use_local_profile(cx);
    let profile = use_ref(cx, || None);
    let user_id = UserId::from_str(user_id).ok().unwrap_or_default();

    use_effect(cx, (&user_id,), |(user_id,)| {
        to_owned![api_client, post_manager, profile, toaster];
        async move {
            post_manager.write().clear();
            use rustter_endpoint::{ViewProfile, ViewProfileOk};

            let request = ViewProfile { for_user: user_id };

            let response = post_json!(<ViewProfileOk>, api_client, request);
            match response {
                Ok(res) => {
                    profile.with_mut(|profile| *profile = Some(res.profile));
                    post_manager.write().populate(res.posts.into_iter());
                }
                Err(e) => {
                    toaster
                        .write()
                        .error(format!("Fetching public posts failed: {e}"), None);
                }
            }
        }
    });

    let follow_onclick = async_handler!(&cx, [api_client, toaster, profile], move |_| async move {
        use rustter_endpoint::user::{
            endpoint::{Follow, FollowOk},
            types::FollowAction,
        };

        let am_following = match profile.read().as_ref() {
            Some(profile) => profile.am_following,
            None => false,
        };

        let action = match am_following {
            true => FollowAction::Unfollow,
            false => FollowAction::Follow,
        };

        let request = Follow { user_id, action };

        match post_json!(<FollowOk>,api_client, request) {
            Ok(res) => {
                profile.with_mut(|profile| {
                    profile.as_mut().map(|p| p.am_following = res.status.into())
                });
            }
            Err(e) => {
                toaster
                    .write()
                    .error(format!("Failed to update follow status {}", e), None);
            }
        };
    });

    let ProfileSection = {
        match profile.with(|profile| profile.clone()) {
            Some(profile) => {
                let display_name = profile
                    .display_name
                    .map(|name| name.into_inner())
                    .unwrap_or_else(|| "(None)".to_string());
                let profile_image = profile
                    .profile_image
                    .map(|url| url.to_string())
                    .unwrap_or_else(|| "".to_string());
                let follow_button_text = match profile.am_following {
                    true => "Unfollow",
                    false => "Follow",
                };

                rsx! {
                    div {
                        class: "flex flex-col gap-3 mt-[var(--appbar-height)]",
                        div {
                            class: "flex flex-row justify-center",
                            img {
                                class: "profile-portrait-lg",
                                src:"{profile_image}"
                            }
                        },
                        div { "Handle: {profile.handle}"},
                        div {"Name: {display_name}"},
                        button {
                            class: "btn",
                            onclick: follow_onclick,
                            "{follow_button_text}"
                        }
                    }
                }
            }
            None => rsx! {
                span {
                class: "mt-[var(--appbar-height)] text-center",
                "Loading profile..."
                }
            },
        }
    };

    let Posts = post_manager.read().all_to_public();

    render! {
        AppBar {
            title: "View profile",
            buttons: vec![
                (
            AppBarRoute::GoBack,
            "/static/icons/icon-back.svg",
            "Back",
            "Go to previous page",
        ),
            ]
        },
        ProfileSection,
        div {
            class: "font-bold text-center my-6",
            "Posts"
        },
        hr {
            class: "h-px my-6 bg-gray-200 border-0"
        },
        Posts.into_iter()
    }
}
