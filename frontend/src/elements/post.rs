#![allow(non_snake_case)]

mod Content;
use Content::Content;
mod PublicPostEntry;
pub use PublicPostEntry::PublicPostEntry;
mod ActionBar;
mod Header;

pub use ActionBar::ActionBar;
pub use Header::Header;

use dioxus::prelude::*;
use fermi::{use_atom_ref, UseAtomRef};
use indexmap::IndexMap;
use rustter_domain::ids::PostId;
use rustter_endpoint::post::types::PublicPost;

#[derive(Default)]
pub struct PostManager {
    pub posts: IndexMap<PostId, PublicPost>,
}

impl PostManager {
    pub fn update<F>(&mut self, id: PostId, mut f: F) -> bool
    where
        F: FnMut(&mut PublicPost),
    {
        if let Some(post) = self.posts.get_mut(&id) {
            f(post);
            true
        } else {
            false
        }
    }

    pub fn populate<T: Iterator<Item = PublicPost>>(&mut self, posts: T) {
        self.posts.clear();
        // sort by time posted

        for post in posts {
            self.posts.insert(post.id, post);
        }
    }

    pub fn clear(&mut self) {
        self.posts.clear();
    }
    pub fn get(&self, id: &PostId) -> Option<&PublicPost> {
        self.posts.get(id)
    }
    pub fn remove(&mut self, id: PostId) {
        self.posts.remove(&id);
    }

    pub fn all_to_public<'a, 'b>(&self) -> Vec<LazyNodes<'a, 'b>> {
        self.posts
            .iter()
            .map(|(&id, _)| {
                rsx! { PublicPostEntry { post_id: id } }
            })
            .collect()
    }
}

pub fn use_post_manager(cx: &ScopeState) -> &UseAtomRef<PostManager> {
    use_atom_ref(cx, &crate::app::POST_MANAGER)
}
