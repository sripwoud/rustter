use crate::handler::{with_handler, with_public_handler};
use crate::AppState;
use axum::http::HeaderValue;
use axum::routing::{get, post};
use axum::{Extension, Router};
use hyper::header::CONTENT_TYPE;
use hyper::Method;
use rustter_endpoint::post::endpoint::NewPost;
use rustter_endpoint::{CreateUser, Login};
use rustter_endpoint::{Endpoint, TrendingPosts};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

pub fn new_router(state: AppState) -> axum::Router {
    let public_routes = Router::new()
        .route("/", get(move || async { "this is the root route" }))
        .route(CreateUser::URL, post(with_public_handler::<CreateUser>))
        .route(Login::URL, post(with_public_handler::<Login>))
        .route(
            TrendingPosts::URL,
            get(with_public_handler::<TrendingPosts>),
        );
    let authorized_routes = Router::new().route(NewPost::URL, post(with_handler::<NewPost>));

    // using layer(ServiceBuilder::new().layer()) execute layers in same order as they are defined
    // instead of layer().layer().layer() which doesn't
    Router::new()
        .merge(public_routes)
        .merge(authorized_routes)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(
                            DefaultOnResponse::new()
                                .level(Level::INFO)
                                .latency_unit(LatencyUnit::Micros),
                        ),
                )
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                        .allow_credentials(std::env::var("FRONTEND_URL").is_ok())
                        .allow_origin(
                            std::env::var("FRONTEND_URL")
                                .unwrap_or(String::from("*"))
                                .parse::<HeaderValue>()
                                .unwrap(),
                        )
                        .allow_headers([CONTENT_TYPE]),
                )
                .layer(Extension(state.clone())), // for layers
        )
        .with_state(state) // for handlers
}
