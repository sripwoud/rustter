#![allow(clippy::redundant_closure_call)]
#![allow(clippy::await_holding_refcell_ref)]
#![allow(clippy::drop_non_drop)]
#![allow(non_snake_case)]

use cfg_if::cfg_if;

pub mod util;
use util::api_client::ApiClient;
pub mod app;
use app::App;
pub mod elements;
pub mod page;

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
    // FIXME: route tree needs to be flattened for the deployment as a static site to work
    // https://dioxuslabs.com/learn/0.4/router/reference/static-generation
    dioxus_web::launch(App)
}

mod prelude {
    pub use crate::elements::{appbar::{self, AppBar, AppBarImgButton::AppBarImgButton}, use_post_manager, use_toaster};
    pub use crate::page::Route;
    pub use crate::util::api_client::{fetch_json, post_json, ApiClient};
    pub use crate::util::{async_handler, maybe_class, sync_handler};
}
