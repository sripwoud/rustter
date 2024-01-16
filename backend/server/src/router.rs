use super::handler::{post, user};
use crate::handler::{load_image, with_json_handler, with_json_public_handler};
use crate::AppState;
use axum::extract::DefaultBodyLimit;
use axum::http::HeaderValue;
use axum::routing::{get, post};
use axum::{Extension, Router};
use hyper::{header::CONTENT_TYPE, Method};
use rustter_endpoint::{
    Bookmark, BookmarkedPosts, Boost, CreateUser, Endpoint, GetMyProfile, HomePosts, LikedPosts,
    Login, NewPost, Reaction, TrendingPosts, UpdateProfile, ViewProfile, Vote,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

const EIGHT_MEGABYTES: usize = 1024 * 1024 * 8;

pub fn new_router(state: AppState) -> Router {
    use rustter_endpoint::app_url::user_content::IMAGE_ROUTE;

    let public_routes = Router::new()
        .route("/", get(move || async { "this is the root route" }))
        .route(
            CreateUser::URL,
            post(with_json_public_handler::<CreateUser>),
        )
        .route(&format!("/{}:id", IMAGE_ROUTE), get(load_image))
        .route(Login::URL, post(with_json_public_handler::<Login>));

    let authorized_routes = Router::new()
        .route(NewPost::URL, post(with_json_handler::<NewPost>))
        .route(HomePosts::URL, get(post::home_posts))
        .route(BookmarkedPosts::URL, get(post::bookmarked_posts))
        .route(LikedPosts::URL, get(post::liked_posts))
        .route(TrendingPosts::URL, get(post::trending_posts))
        .route(Bookmark::URL, post(with_json_handler::<Bookmark>))
        .route(Boost::URL, post(with_json_handler::<Boost>))
        .route(Vote::URL, post(with_json_handler::<Vote>))
        .route(Reaction::URL, post(with_json_handler::<Reaction>))
        .route(ViewProfile::URL, get(with_json_handler::<ViewProfile>))
        .route(GetMyProfile::URL, get(user::get_my_profile))
        .route(UpdateProfile::URL, post(with_json_handler::<UpdateProfile>))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(EIGHT_MEGABYTES));
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
