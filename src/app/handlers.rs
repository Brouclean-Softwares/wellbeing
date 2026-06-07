use crate::AppState;
use crate::app::templates::journal::JournalOfDayPage;
use crate::app::templates::{WelcomePage, WelcomeWithLovePage};
use crate::data::users::{ConnectedProfile, Profile};
use crate::errors::AppError;
use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Deserialize;

pub mod journal;
pub mod satisfying_moments;
pub mod users;

pub fn init_router() -> Router<AppState> {
    Router::new()
        .nest("/journal", journal::init_router())
        .nest("/satisfying_moments", satisfying_moments::init_router())
        .nest("/users", users::init_router())
        .route("/", get(home))
        .route("/welcome_user_with_love", get(welcome_user_with_love))
}

#[derive(Deserialize)]
pub struct DeleteForm {
    pub id: i64,
}

pub async fn home(profile: Profile) -> Result<impl IntoResponse, AppError> {
    if profile.user.is_some() {
        let connected_profile = ConnectedProfile::try_from(profile.clone())?;

        Ok(JournalOfDayPage::from(connected_profile, None).into_response())
    } else {
        Ok(WelcomePage::from(profile).into_response())
    }
}

pub async fn welcome_user_with_love(connected_profile: ConnectedProfile) -> WelcomeWithLovePage {
    WelcomeWithLovePage::from(connected_profile)
}
