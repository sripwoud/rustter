use crate::error::ApiResult;
use crate::extractor::{DbConnection::DbConnection, UserSession::UserSession};
use crate::handler::{save_image, AuthorizedApiRequest, PublicApiRequest};
use crate::AppState;
use axum::{async_trait, http::StatusCode, Json};
use chrono::Duration;
use rustter_domain::ids::{ImageId, UserId};
use rustter_domain::user::DisplayName;
use rustter_endpoint::user::types::{FollowAction, PublicUserProfile};
use rustter_endpoint::{
    CreateUser, CreateUserOk, Follow, FollowOk, GetMyProfileOk, Login, LoginOk, Update,
    UpdateProfile, UpdateProfileOk, ViewProfile, ViewProfileOk,
};
use rustter_query::{
    session::{self, Session},
    user::{UpdateProfileParams, User},
    AsyncConnection,
};
use tracing::{info, span};

#[derive(Clone)]
struct SessionSignature(String);

fn new_session(
    state: &AppState,
    conn: &mut AsyncConnection,
    user_id: UserId,
) -> ApiResult<(Session, SessionSignature)> {
    let fingerprint = serde_json::json!({});
    // TODO extract in Session::new()?
    let duration = Duration::weeks(3);
    let session = session::new(conn, user_id, duration, fingerprint.into())?;

    let mut rng = state.rng.clone();
    let signature = state
        .signing_keys
        .sign(&mut rng, session.id.as_uuid().as_bytes());
    let signature = rustter_crypto::encode_base64(signature);

    Ok((session, SessionSignature(signature)))
}

#[async_trait]
impl PublicApiRequest for CreateUser {
    type Response = (StatusCode, Json<CreateUserOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let password_hash = rustter_crypto::hash_password(&self.password)?;
        let user_id = rustter_query::user::new(&mut conn, password_hash, &self.username)?;

        info!(target:"rustter_server",username = self.username.as_ref(), "new user created");

        let (session, signature) = new_session(&state, &mut conn, user_id)?;

        Ok((
            StatusCode::CREATED,
            Json(CreateUserOk {
                session_signature: signature.0,
                session_id: session.id,
                session_expires: session.expires_at,

                user_id,
                username: self.username,
            }),
        ))
    }
}

#[async_trait]
impl PublicApiRequest for Login {
    type Response = (StatusCode, Json<LoginOk>);

    async fn process_request(
        self,
        DbConnection(mut conn): DbConnection,
        state: AppState,
    ) -> ApiResult<Self::Response> {
        let _span = span!(
            tracing::Level::INFO, "logging in",
        user = %self.username.as_ref())
        .entered();

        let password_hash = rustter_query::user::get_password_hash(&mut conn, &self.username)?;
        let password_hash = rustter_crypto::password::deserialize_hash(&password_hash)?;
        rustter_crypto::verify_password(self.password, &password_hash)?;

        let user = rustter_query::user::find(&mut conn, &self.username)?;

        let (session, signature) = new_session(&state, &mut conn, user.id)?;

        Ok((
            StatusCode::OK,
            Json(LoginOk {
                session_signature: signature.0,
                session_id: session.id,
                session_expires: session.expires_at,

                display_name: user.display_name,
                email: user.email,
                profile_image: None,
                user_id: user.id,
            }),
        ))
    }
}

pub fn to_public(user: User) -> ApiResult<PublicUserProfile> {
    Ok(PublicUserProfile {
        id: user.id,
        // TODO: should not have to clone
        display_name: user
            .display_name
            .clone()
            .and_then(|s| DisplayName::new(s).ok()),
        // TODO: should not have to clone
        handle: user.handle.clone(),
        profile_image: user.profile_image_url_from_id(),
        created_at: user.created_at,
        // TODO
        am_following: false,
    })
}

pub async fn get_my_profile(
    DbConnection(mut conn): DbConnection,
    session: UserSession,
) -> ApiResult<(StatusCode, Json<GetMyProfileOk>)> {
    info!(target:"rustter_server", user_id=session.user_id.to_string(), "getting authed user profile");

    let user = rustter_query::user::get(&mut conn, session.user_id)?;
    let profile_image = user.profile_image_url_from_id();

    Ok((
        StatusCode::OK,
        Json(GetMyProfileOk {
            display_name: user.display_name,
            email: user.email,
            profile_image,
            user_id: user.id,
        }),
    ))
}

#[async_trait]
impl AuthorizedApiRequest for ViewProfile {
    type Response = (StatusCode, Json<ViewProfileOk>);

    async fn process_request(
        mut self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        info!(target:"rustter_server", user_id=self.for_user.to_string(), "viewing user profile");

        let profile = rustter_query::user::get(&mut conn, self.for_user)?;
        let profile = to_public(profile)?;

        let posts = super::post::public_posts(DbConnection(conn), Some(&session), self.for_user)?;

        Ok((StatusCode::OK, Json(ViewProfileOk { profile, posts })))
    }
}

#[async_trait]
impl AuthorizedApiRequest for UpdateProfile {
    type Response = (StatusCode, Json<UpdateProfileOk>);

    async fn process_request(
        mut self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        info!(target:"rustter_server", "updating user profile");

        let password_hash = {
            if let Update::Change(ref password) = self.password {
                Update::Change(rustter_crypto::hash_password(password)?)
            } else {
                Update::NoChange
            }
        };

        if let Update::Change(ref img) = self.profile_image {
            let id = ImageId::new();
            save_image(id, img).await?;
        };

        let query_params = UpdateProfileParams {
            id: session.user_id,
            display_name: self.display_name,
            email: self.email,
            password_hash,
            profile_image: self.profile_image.clone(),
        };

        rustter_query::user::update_profile(&mut conn, query_params)?;
        let user = rustter_query::user::get(&mut conn, session.user_id)?;
        let profile_image = user.profile_image_url_from_id();

        Ok((
            StatusCode::OK,
            Json(UpdateProfileOk {
                display_name: user.display_name,
                email: user.email,
                profile_image,
                user_id: user.id,
            }),
        ))
    }
}

#[async_trait]
impl AuthorizedApiRequest for Follow {
    type Response = (StatusCode, Json<FollowOk>);

    async fn process_request(
        mut self,
        DbConnection(mut conn): DbConnection,
        session: UserSession,
        _state: AppState,
    ) -> ApiResult<Self::Response> {
        info!(target:"rustter_server","follow action");

        match self.action {
            FollowAction::Follow => {
                rustter_query::follow::follow(&mut conn, session.user_id, self.user_id)?;
            }
            FollowAction::Unfollow => {
                rustter_query::follow::unfollow(&mut conn, session.user_id, self.user_id)?;
            }
        }

        Ok((
            StatusCode::OK,
            Json(FollowOk {
                status: self.action,
            }),
        ))
    }
}
