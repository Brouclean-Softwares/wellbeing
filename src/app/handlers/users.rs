use crate::AppState;
use crate::app::templates::users::UserPage;
use crate::data::users::{ConnectedProfile, User};
use crate::errors::AppError;
use axum::extract::{Query, State};
use axum::response::Redirect;
use axum::routing::get;
use axum::{Form, Router};
use serde::Deserialize;

pub fn init_router() -> Router<AppState> {
    Router::new().route("/user", get(user).post(update))
}

#[derive(Deserialize)]
struct UserParams {
    id: Option<i64>,
}

async fn user(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Query(params): Query<UserParams>,
) -> Result<UserPage, AppError> {
    let id = params.id.unwrap_or(connected_profile.user.id);

    match User::select_by_id(&app_state, Some(id)).await? {
        Some(user) => {
            if user == connected_profile.user || connected_profile.is_admin {
                Ok(UserPage::from(app_state, connected_profile, user))
            } else {
                Err(AppError::Unauthorized(connected_profile.into()))
            }
        }

        None => Err(AppError::Unauthorized(connected_profile.into())),
    }
}

#[derive(Deserialize)]
struct UpdateForm {
    id: i64,
    preferred_language: Option<String>,
}

async fn update(
    State(app_state): State<AppState>,
    connected_profile: ConnectedProfile,
    Form(form): Form<UpdateForm>,
) -> Result<Redirect, Redirect> {
    let redirect = Redirect::to(&format!("/users/user?id={}", form.id));

    let user = User::select_by_id(&app_state, Some(form.id))
        .await
        .or_else(|error| Err(error.log_and_redirect(redirect.clone())))?;

    if let Some(mut user) = user {
        user.preferred_language = form.preferred_language;

        user.update(&app_state, &connected_profile)
            .await
            .or_else(|error| Err(error.log_and_redirect(redirect.clone())))?;
    }

    Ok(redirect)
}
