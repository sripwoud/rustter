#![allow(clippy::redundant_closure_call)]
#![allow(clippy::await_holding_refcell_ref)]
#![allow(clippy::drop_non_drop)]
#![allow(non_snake_case)]

pub mod util;

pub mod app;
pub mod elements;
pub mod page;

use crate::util::ApiClient;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

fn main() {
    init_log();
    ApiClient::init();
    dioxus_web::launch(app::App)
}

mod prelude {
    pub use crate::page::Route;
    pub use crate::util::api_client::{fetch_json, ApiClient};
    pub use crate::util::{async_handler, maybe_class, sync_handler};
}
