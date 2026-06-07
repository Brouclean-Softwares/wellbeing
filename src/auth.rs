use crate::AppState;
use crate::app::templates::users::UserPage;
use crate::data::sessions::Session;
use crate::data::users::{ConnectedProfile, MayBeUser};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{
    Form, Router,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::PrivateCookieJar;
use axum_extra::extract::cookie::Cookie;
use serde::Deserialize;

pub mod google;

pub const SESSION_TOKEN: &str = "sid";
pub const REDIRECT_URI_AFTER_AUTH: &str = "redir_auth";

pub fn init_router() -> Router<AppState> {
    Router::new()
        .route("/profile", get(profile))
        .route("/sign_in", post(sign_in))
        .route("/sign_out", get(sign_out))
        .route("/google_callback", get(google::callback))
}

pub async fn profile(profile: ConnectedProfile) -> impl IntoResponse {
    UserPage::from(profile).into_response()
}

#[derive(Deserialize)]
pub struct SignInForm {
    pub redirection_uri: String,
}

pub async fn sign_in(
    State(app_state): State<AppState>,
    MayBeUser(profile): MayBeUser,
    cookie_jar: PrivateCookieJar,
    Form(form): Form<SignInForm>,
) -> impl IntoResponse {
    let redirection_uri_when_connected = form.redirection_uri;

    if profile.is_some() {
        Redirect::to(&redirection_uri_when_connected).into_response()
    } else {
        let url = google::connection_url(&app_state);

        let cookie_jar = cookie_jar.add(Cookie::new(
            REDIRECT_URI_AFTER_AUTH,
            redirection_uri_when_connected,
        ));

        (cookie_jar, Redirect::to(&url)).into_response()
    }
}

pub async fn sign_out(
    State(app_state): State<AppState>,
    cookie_jar: PrivateCookieJar,
) -> impl IntoResponse {
    if let Some(token) = cookie_jar.get(SESSION_TOKEN).map(|c| c.value().to_owned()) {
        let _ = Session::delete_for_token(&app_state, &token).await;
    }

    (
        cookie_jar
            .clone()
            .remove(Cookie::build(SESSION_TOKEN).path("/")),
        Redirect::to("/"),
    )
}
