#![allow(non_snake_case)]

use chrono::{DateTime, Duration, Utc};
use dioxus::prelude::*;
use fermi::{use_atom_ref, UseAtomRef};
use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum ToastKind {
    Success,
    Error,
    // Warning,
    Info,
}

#[derive(Debug)]
pub struct Toast {
    kind: ToastKind,
    message: String,
    expires: DateTime<Utc>,
}

impl Toast {
    pub fn new(kind: ToastKind, message: String, expires: Duration) -> Self {
        Self {
            kind,
            message,
            expires: Utc::now() + expires,
        }
    }
}

#[derive(Debug, Default)]
pub struct Toaster {
    toasts: HashMap<usize, Toast>,
    next_id: usize,
}

impl Toaster {
    fn increment_id(&mut self) {
        self.next_id += 1;
    }
    fn push<T: Into<String>>(
        &mut self,
        toast_kind: ToastKind,
    ) -> impl FnMut(T, Option<Duration>) + '_ {
        move |message, duration| {
            let toast = match duration {
                Some(expires) => Toast::new(toast_kind, message.into(), expires),
                None => Toast::new(toast_kind, message.into(), Duration::seconds(5)),
            };
            self.toasts.insert(self.next_id, toast);
            self.increment_id();
        }
    }

    pub fn success<T: Into<String>>(&mut self, message: T, duration: Option<Duration>) {
        self.push(ToastKind::Success)(message, duration);
    }

    pub fn error<T: Into<String>>(&mut self, message: T, duration: Option<Duration>) {
        self.push(ToastKind::Error)(message, duration);
    }

    pub fn info<T: Into<String>>(&mut self, message: T, duration: Option<Duration>) {
        self.push(ToastKind::Info)(message, duration);
    }

    pub fn remove(&mut self, id: usize) {
        self.toasts.remove(&id);
    }

    pub fn iter(&self) -> Iter<'_, usize, Toast> {
        self.toasts.iter()
    }

    pub fn remove_all_expired(&mut self) {
        let now = Utc::now();
        self.toasts.retain(|_, toast| toast.expires > now);
    }
}

pub fn use_toaster(cx: &ScopeState) -> &UseAtomRef<Toaster> {
    use_atom_ref(cx, &crate::app::TOASTER)
}

#[derive(Props)]
pub struct ToastRootProps<'a> {
    toaster: &'a UseAtomRef<Toaster>,
}

pub fn ToastRoot<'a>(cx: Scope<'a, ToastRootProps<'a>>) -> Element {
    let toaster = cx.props.toaster;
    let toasts = toaster.read();
    let toasts = toasts.iter().map(|(&id, toast)| {
        let toast_style = match toast.kind {
            ToastKind::Success => "bg-emerald-200",
            ToastKind::Error => "bg-rose-200",
            ToastKind::Info => "bg-blue-200",
        };

        rsx! {
            div {
                key: "{id}",
                class: "{toast_style} p-3 cursor-pointer border-solid border rounded",
                onclick: move |_| toaster.write().remove(id),
                "{toast.message}"
            }
        }
    });

    let total_toasts = &toaster.read().toasts.len();

    // async hook that'll run everytime the total_toasts changes
    // the loop ticks every 200ms until total_toasts is 0
    use_future(cx, (total_toasts,), move |_| {
        let toaster = toaster.clone();

        async move {
            while !toaster.read().toasts.is_empty() {
                toaster.write().remove_all_expired();
                gloo_timers::future::TimeoutFuture::new(200_u32).await;
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "fixed bottom-[var(--navbar-height)] w-screen max-w[var(--content-max-width)]",
            div {
                class: "flex flex-col gap-5 px-5 mb-5",
                toasts
            }
        }

    })
}
