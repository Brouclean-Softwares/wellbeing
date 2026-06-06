use crate::AppState;
use crate::app::templates::{HomePage, WelcomePage};
use crate::data::users::{ConnectedProfile, Profile};
use crate::errors::AppError;
use axum::extract::State;
use axum::response::IntoResponse;

pub mod satisfying_moments;
pub mod users;

pub async fn home_page(
    State(app_state): State<AppState>,
    profile: Profile,
) -> Result<impl IntoResponse, AppError> {
    if profile.user.is_some() {
        let connected_profile = ConnectedProfile::try_from(profile.clone())?;

        Ok(HomePage::get(&app_state, &connected_profile).into_response())
    } else {
        Ok(WelcomePage::get(&app_state, &profile).into_response())
    }
}
