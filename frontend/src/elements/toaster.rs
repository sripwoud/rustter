#![allow(non_snake_case)]

use std::collections::hash_map::Iter;
use chrono::{DateTime, Duration, Utc};
use derive_more::Constructor;
use std::collections::HashMap;
use dioxus::prelude::*;
use fermi::{use_atom_ref, UseAtomRef};

#[derive(Clone, Copy)]
pub enum ToastKind {
    Success,
    Error,
    // Warning,
    Info,
}

#[derive(Constructor)]
pub struct Toast {
    id: usize,
    kind: ToastKind,
    message: String,
    expires: DateTime<Utc>,
}

#[derive(Default)]
pub struct Toaster {
    toasts: HashMap<usize, Toast>,
    next_id: usize,
}

impl Toaster {
    pub fn new() -> Self {
        Self {
            toasts: HashMap::new(),
            next_id: 0,
        }
    }

    fn increment_id(&mut self) {
        self.next_id += 1;
    }
    fn push<'a, T:Into<String>>(&'a mut self, toast_kind: ToastKind) -> impl FnMut(T, Duration) + 'a {
        move|message, duration|{
            let toast = Toast::new(self.next_id, toast_kind, message.into(), Utc::now() + duration);
            self.toasts.insert(self.next_id, toast);
            self.increment_id();
        }
    }

    pub fn success<T:Into<String>>(message:T, duration:Duration){
        Self::new().push(ToastKind::Success)(message, duration);
    }

    pub fn error<T:Into<String>>(message:T, duration:Duration){
        Self::new().push(ToastKind::Error)(message, duration);
    }

    pub fn info<T:Into<String>>(message:T, duration:Duration){
        Self::new().push(ToastKind::Info)(message, duration);
    }

    pub fn remove(&mut self, id: usize) {
        self.toasts.remove(&id);
    }

    pub fn iter(&self) -> Iter<'_, usize, Toast> {
        self.toasts.iter()
    }
}

pub fn use_toaster(cx:&ScopeState) -> &UseAtomRef<Toaster> {
    use_atom_ref(cx, &crate::app::TOASTER)
}
pub struct ToastRootProps<'a> {
    toaster: &'a UseAtomRef<Toaster>,
}

pub fn ToastRoot<'a>(cx: Scope<'a, ToastRootProps<'a>>) {
    todo!()
}
