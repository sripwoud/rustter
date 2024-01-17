use crate::prelude::*;
use dioxus::prelude::*;
use itertools::Itertools;
use rustter_domain::ids::{PollChoiceId, PostId};
use rustter_endpoint::post::types;
use std::collections::HashSet;

#[inline_props]
pub fn Poll<'a>(cx: Scope<'a>, post_id: PostId, poll: &'a types::Poll) -> Element {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let vote_onclick = async_handler!(
        &cx,
        [api_client, toaster],
        move |post_id, choice_id| async move {
            use rustter_endpoint::{Vote, VoteOk};

            let vote = Vote { post_id, choice_id };

            match post_json!(<VoteOk>, api_client, vote) {
                Ok(vote) => match vote.cast {
                    types::VoteCast::Yes => {
                        toaster.write().success("Vote susccessfully cast", None)
                    }

                    types::VoteCast::AlreadyVoted => {
                        toaster.write().info("You already voted on this poll", None)
                    }
                },
                Err(e) => toaster
                    .write()
                    .error(format!("Failed to cast vote {}", e), None),
            }
        }
    );

    let total_votes = poll
        .choices
        .iter()
        .fold(0, |acc, choice| acc + choice.num_votes);
    let leader_ids: HashSet<PollChoiceId> = {
        let leaders = poll
            .choices
            .iter()
            .max_set_by(|a, b| a.num_votes.cmp(&b.num_votes));
        HashSet::from_iter(leaders.iter().map(|leader| leader.id))
    };

    let Choices = poll.choices.iter().map(|choice|{
        let percent = if total_votes > 0 {
            let percent = (choice.num_votes as f64 / total_votes as f64) * 100.0;
            format!("{:.0}%", percent)
        } else {
            "0%".to_string()
        };

        let bg_color = if leader_ids.contains(&choice.id) {
            "bg-blue-300"
        } else {
            "bg-neutral-300"
        };
        let bold = maybe_class!("font-bold", leader_ids.contains(&choice.id));

        render! {
            li {
                key: "{choice.id.to_string()}",
                class: "relative p-2 m-2 cursor-pointer grid grid-cols-[3rem_1fr] border rounder border-slate-400",
                onclick: move|_| vote_onclick(*post_id, choice.id),
                div {
                    class: "absolute left-0 {bg_color} h-full rounded z-[-1]",
                    style: "width: {percent}",
                },
                div {
                    class: "{bold}",
                    "{percent}"
                },
                div {
                    class: "{bold}",
                    choice.description.as_ref()
                }
            }
        }


    });

    let Headline = render! { figcaption { poll.headline.as_ref() } };

    render! {
        div {
            Headline
            ul {
                Choices.into_iter()
            }
        }
    }
}
